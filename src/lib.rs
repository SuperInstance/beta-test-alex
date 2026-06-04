//! Beta-test-alex: Integration scenario for SuperInstance ternary agent crates.
//!
//! Simulates evolving NPC behavior strategies for a Roblox-style game.
//! Implements the APIs of ternary-fitness, strategy-ecology, evolution-ternary,
//! and ternary-classifier locally to validate the design.

use rand::Rng;
use std::fmt;

// ── ternary-fitness ──────────────────────────────────────────────────────────

/// A ternary trit value: Negative, Zero, or Positive.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Trit {
    Neg = -1,
    Zero = 0,
    Pos = 1,
}

impl Trit {
    pub fn from_i8(v: i8) -> Self {
        match v {
            -1 => Trit::Neg,
            0 => Trit::Zero,
            1 => Trit::Pos,
            _ => Trit::Zero,
        }
    }

    pub fn random(rng: &mut impl Rng) -> Self {
        match rng.gen_range(0..3) {
            0 => Trit::Neg,
            1 => Trit::Zero,
            _ => Trit::Pos,
        }
    }
}

/// A ternary genome — a vector of trits representing an NPC behavior strategy.
#[derive(Clone, Debug)]
pub struct TernaryGenome {
    pub genes: Vec<Trit>,
}

impl TernaryGenome {
    pub fn random(len: usize, rng: &mut impl Rng) -> Self {
        Self {
            genes: (0..len).map(|_| Trit::random(rng)).collect(),
        }
    }

    pub fn fitness(&self) -> f64 {
        // ternary-fitness: sum of trit values, normalized to [0, 1]
        let raw: f64 = self.genes.iter().map(|t| (*t as i8) as f64).sum();
        let max = self.genes.len() as f64;
        (raw + max) / (2.0 * max) // map [-max, +max] → [0, 1]
    }

    pub fn mutate(&self, rate: f64, rng: &mut impl Rng) -> Self {
        let genes = self.genes.iter().map(|&g| {
            if rng.gen::<f64>() < rate {
                Trit::random(rng)
            } else {
                g
            }
        }).collect();
        Self { genes }
    }

    pub fn len(&self) -> usize {
        self.genes.len()
    }
}

// ── strategy-ecology ─────────────────────────────────────────────────────────

/// Species tag for ecological classification.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Species {
    Aggressive,  // mostly Pos
    Defensive,   // mostly Neg
    Balanced,    // mostly Zero
    Hybrid,      // mixed
}

/// Classify a genome into an ecological species.
pub fn classify_species(genome: &TernaryGenome) -> Species {
    let pos = genome.genes.iter().filter(|t| **t == Trit::Pos).count();
    let neg = genome.genes.iter().filter(|t| **t == Trit::Neg).count();
    let total = genome.genes.len();
    let pos_ratio = pos as f64 / total as f64;
    let neg_ratio = neg as f64 / total as f64;

    if pos_ratio > 0.6 {
        Species::Aggressive
    } else if neg_ratio > 0.6 {
        Species::Defensive
    } else if pos_ratio < 0.3 && neg_ratio < 0.3 {
        Species::Balanced
    } else {
        Species::Hybrid
    }
}

// ── evolution-ternary ────────────────────────────────────────────────────────

/// An agent in the population.
#[derive(Clone, Debug)]
pub struct Agent {
    pub genome: TernaryGenome,
    pub species: Species,
    pub fitness: f64,
}

impl Agent {
    pub fn new(genome: TernaryGenome) -> Self {
        let species = classify_species(&genome);
        let fitness = genome.fitness();
        Self { genome, species, fitness }
    }
}

/// A population of evolving agents.
pub struct Population {
    pub agents: Vec<Agent>,
    pub generation: u32,
    pub genome_len: usize,
}

impl Population {
    pub fn new(size: usize, genome_len: usize) -> Self {
        let mut rng = rand::thread_rng();
        let agents = (0..size)
            .map(|_| Agent::new(TernaryGenome::random(genome_len, &mut rng)))
            .collect();
        Self { agents, generation: 0, genome_len }
    }

    /// Tournament selection — pick the best of `k` random agents.
    fn select_parent(&self, k: usize) -> &Agent {
        let mut rng = rand::thread_rng();
        (0..k)
            .map(|_| &self.agents[rng.gen_range(0..self.agents.len())])
            .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
            .unwrap()
    }

    /// Crossover two parents to produce a child genome.
    pub fn crossover(p1: &TernaryGenome, p2: &TernaryGenome) -> TernaryGenome {
        debug_assert_eq!(p1.len(), p2.len(), "Genome length mismatch in crossover");
        let mut rng = rand::thread_rng();
        let point = rng.gen_range(0..p1.genes.len());
        let genes = p1.genes[..point]
            .iter()
            .chain(p2.genes[point..].iter())
            .copied()
            .collect();
        TernaryGenome { genes }
    }

    /// Evolve one generation using selection, crossover, and mutation.
    pub fn evolve(&mut self) {
        let mut rng = rand::thread_rng();
        let size = self.agents.len();

        // Elitism: keep top 10%
        let mut sorted = self.agents.clone();
        sorted.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        let elite_count = (size as f64 * 0.1).ceil() as usize;

        let mut next_gen: Vec<Agent> = sorted[..elite_count].to_vec();

        while next_gen.len() < size {
            let p1 = self.select_parent(3);
            let p2 = self.select_parent(3);
            let child_genome = TernaryGenome::mutate(
                &Self::crossover(&p1.genome, &p2.genome),
                0.05,
                &mut rng,
            );
            next_gen.push(Agent::new(child_genome));
        }

        self.agents = next_gen;
        self.generation += 1;
    }

    /// Best agent in the population.
    pub fn best(&self) -> &Agent {
        self.agents.iter().max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap()).unwrap()
    }

    /// Average fitness.
    pub fn avg_fitness(&self) -> f64 {
        self.agents.iter().map(|a| a.fitness).sum::<f64>() / self.agents.len() as f64
    }

    /// Species distribution counts: [Aggressive, Defensive, Balanced, Hybrid]
    pub fn species_distribution(&self) -> [usize; 4] {
        let mut counts = [0usize; 4];
        for a in &self.agents {
            match a.species {
                Species::Aggressive => counts[0] += 1,
                Species::Defensive => counts[1] += 1,
                Species::Balanced => counts[2] += 1,
                Species::Hybrid => counts[3] += 1,
            }
        }
        counts
    }
}

// ── ternary-classifier ───────────────────────────────────────────────────────

/// Full classification report.
pub struct ClassificationReport {
    pub total: usize,
    pub species_counts: [usize; 4],
    pub avg_fitness: f64,
    pub best_fitness: f64,
    pub best_species: Species,
    pub best_genome: TernaryGenome,
}

impl ClassificationReport {
    pub fn from_population(pop: &Population) -> Self {
        let best = pop.best();
        Self {
            total: pop.agents.len(),
            species_counts: pop.species_distribution(),
            avg_fitness: pop.avg_fitness(),
            best_fitness: best.fitness,
            best_species: best.species,
            best_genome: best.genome.clone(),
        }
    }
}

impl fmt::Display for ClassificationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Ternary Classification Report ===")?;
        writeln!(f, "Population:       {}", self.total)?;
        writeln!(f, "Avg Fitness:      {:.4}", self.avg_fitness)?;
        writeln!(f, "Best Fitness:     {:.4}", self.best_fitness)?;
        writeln!(f, "Best Species:     {:?}", self.best_species)?;
        writeln!(f, "Species Distribution:")?;
        writeln!(f, "  Aggressive: {}", self.species_counts[0])?;
        writeln!(f, "  Defensive:  {}", self.species_counts[1])?;
        writeln!(f, "  Balanced:   {}", self.species_counts[2])?;
        writeln!(f, "  Hybrid:     {}", self.species_counts[3])?;
        writeln!(f, "Best Genome: {:?}", self.best_genome.genes.iter().map(|t| *t as i8).collect::<Vec<_>>())?;
        Ok(())
    }
}
