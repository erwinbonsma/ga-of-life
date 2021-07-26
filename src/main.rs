use ga_of_life::{MyEvolutionaryAlgorithm};

fn main() {
    let mut ga = MyEvolutionaryAlgorithm::new();
    let mut max_fitness = 0.0;

    for _ in 0..1000 {
        ga.step();

        if let Some(stats) = ga.ea().get_stats() {
            println!("max = {}, avg = {}", stats.max_fitness, stats.avg_fitness);
            if stats.max_fitness > max_fitness {
                max_fitness = stats.max_fitness;
                println!("{:?}", stats.best_indiv);
            }
        }
    }

    println!("{:?}", ga.ea().evaluator())
}
