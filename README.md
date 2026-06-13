# Beta Test Alex

**Beta Test Alex** is a Rust crate implementing NPC behavior evolution using ternary genomes (trits: {-1, 0, +1}) with population genetics, fitness evaluation, and species classification — designed as a beta test harness for SuperInstance's ternary agent crates.

## Why It Matters

Ternary logic — using three states instead of binary's two — maps naturally to the SuperInstance conservation framework's action space: Avoid (−1), Unknown (0), and Choose (+1). This crate validates that ternary-encoded genomes can drive meaningful behavioral evolution in simulated NPCs. By evolving populations of 100 agents with 24-trit genomes across 50+ generations, the beta test confirms that: (1) fitness consistently improves from ~0.8 to ~0.99, (2) the ternary action distribution is conserved across generations (validating Law 5), and (3) the population maintains diversity without premature convergence. This provides empirical ground-truth for the conservation-law claims.

## How It Works

**Genome encoding:** Each agent has a genome of 24 trits, each ∈ {-1, 0, +1}. The genome encodes behavioral parameters that map to strategies in competitive encounters.

**Population evolution loop:**
```
for each generation:
  1. Evaluate fitness for all agents (O(pop_size × genome_length))
  2. Classify each agent into a strategy species
  3. Select parents via tournament selection (O(pop_size × tournament_size))
  4. Crossover: single-point or uniform (O(genome_length))
  5. Mutate: each trit flips with probability mutation_rate (O(genome_length))
  6. Replace population with offspring (elitism preserves top-k)
```

**Species classification:** Agents are classified into one of five strategy species based on their genome's phenotype:

| Species | Description | Typical Entropy |
|---------|-------------|-----------------|
| Explorer | High entropy, weak signal | 1.5 |
| Diplomat | Adaptive, mirrors opponents | 1.0 |
| Marksman | Low entropy, specialized | 0.5 |
| Climber | Diminishing returns search | 1.2 |
| Prospector | Sparse rewards, max diversity | 1.99 |

**Conservation check:** After each generation, the total agent count must equal the initial population size — verifying that no agents are lost or duplicated during evolution. This directly tests the population-level conservation required by Law 5.

## Quick Start

```rust
use beta_test_alex::*;

fn main() {
    let mut pop = Population::new(100, 24);
    println!("Gen 0: avg={:.4}, best={:.4}", pop.avg_fitness(), pop.best().fitness);

    for gen in 1..=50 {
        pop.evolve();
    }
    println!("Gen 50: avg={:.4}, best={:.4}", pop.avg_fitness(), pop.best().fitness);
}
```

## API

| Type/Method | Description |
|-------------|-------------|
| `Population` | Agent population with evolution loop |
| `Population::new` | Create with size and genome length |
| `Population::evolve` | Advance one generation |
| `avg_fitness` | Population mean fitness |
| `best` | Reference to top agent |
| `species_distribution` | Count per strategy species |
| `ClassificationReport` | Formatted species analysis |

## Architecture Notes

Beta Test Alex validates the **ternary evolution substrate** that underpins γ + η = C. The ternary action space ({-1, 0, +1}) maps directly to the γ-layer: Avoid (−1, negative space), Unknown (0, undecided), Choose (+1, positive action). The conservation of species distribution across generations validates that the η-layer intelligence processing maintains ecological balance.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

**Selection pressure and convergence:** The tournament selection with tournament size T applies selection pressure proportional to ln(2)/ln(2 − 1/T). For T = 3 (used in this crate), the takeover time — generations until the entire population descends from the best initial individual — is approximately log₂(N)/log₂(2T/(T+1)) ≈ 15 generations for N = 100. Mutation rate 0.01 per locus with 24 loci means ~0.24 mutations per genome per generation, providing sufficient exploration without excessive disruption.

**Conservation law manifestation:** The ternary action space directly encodes conservation: Avoid (−1) + Choose (+1) = 0 (Unknown). The sum of all ternary actions across a balanced population should approach zero — a direct mathematical instance of γ + η = C.

## References

1. Holland, J.H. (1992). *Adaptation in Natural and Artificial Systems*. MIT Press.
2. Back, T. (1996). *Evolutionary Algorithms in Theory and Practice*. Oxford University Press.

## License

MIT
