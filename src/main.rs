use beta_test_alex::*;

fn main() {
    println!("🎮 Beta-Test-Alex: NPC Behavior Evolution");
    println!("==========================================\n");

    let genome_len = 24; // 24 trits = 24 behavior parameters
    let pop_size = 100;
    let generations = 50;

    println!("Creating population of {} agents (genome length: {})...", pop_size, genome_len);
    let mut pop = Population::new(pop_size, genome_len);

    println!("Generation 0 — avg fitness: {:.4}, best: {:.4}, species: {:?}\n",
        pop.avg_fitness(),
        pop.best().fitness,
        pop.species_distribution(),
    );

    // Evolve for 50 generations
    for gen in 1..=generations {
        pop.evolve();
        if gen % 10 == 0 || gen == generations {
            println!("Gen {:3} — avg: {:.4}, best: {:.4}, species: {:?}",
                gen, pop.avg_fitness(), pop.best().fitness, pop.species_distribution());
        }
    }

    // Classify final population
    let report = ClassificationReport::from_population(&pop);
    println!("\n{}", report);

    // Conservation check
    let dist = pop.species_distribution();
    let total: usize = dist.iter().sum();
    println!("Conservation check: {} agents accounted for (expected {})", total, pop_size);
    assert_eq!(total, pop_size, "Conservation law violated!");

    println!("\n✅ Beta test complete!");
}
