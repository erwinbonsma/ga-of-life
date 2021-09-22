use ga_of_life::{MyEvolutionaryAlgorithm, MyEaSettings};

fn main() {
    let ga_settings = MyEaSettings::new();
    let mut ga = MyEvolutionaryAlgorithm::new(&ga_settings);
    let mut max_fitness = 0.0;

    for _ in 0..100 {
        ga.step();

        if let Some(stats) = ga.ea().get_population_stats() {
            println!("max = {}, avg = {}", stats.max_fitness, stats.avg_fitness);
            if stats.max_fitness > max_fitness {
                max_fitness = stats.max_fitness;
                println!("{:?}", stats.best_indiv);
            }
        }
    }

    println!("{:?}", ga.ea().get_stats())
}
