use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod ca;
pub mod ga;

use std::fmt;
use ca::{BitGrid, GameOfLife, GameOfLifeRunner};
use ga::{
    EvolutionaryAlgorithm,
    Phenotype,
    Expressor,
    Evaluator,
    GenotypeFactory,
    GenotypeManipulation,
    GenotypeConfig,
    Mutation,
    Recombination,
    PopulationStats,
};
use ga::binary::{
    BinaryChromosome,
    BinaryBitMutation,
    BinaryNPointBitCrossover
};
use ga::selection::{
//    ElitismSelection,
    TournamentSelection,
};

const GARDEN_SIZE: usize = 64;
const SEED_PATCH_SIZE: usize = 8;

struct MyExpressor {}

#[derive(Hash, Eq, PartialEq)]
pub struct MyPhenotype {
    bit_grid: BitGrid,
}

struct MyEvaluator {
    gol: GameOfLife,
    gol_runner: GameOfLifeRunner,
}

#[derive(fmt::Debug)]
struct MyConfig {
    mutation: BinaryBitMutation,
    recombination: BinaryNPointBitCrossover,
}

#[wasm_bindgen]
pub struct MyEvolutionaryAlgorithm {
    ea: EvolutionaryAlgorithm<BinaryChromosome, MyPhenotype>,
    population_stats: Option<PopulationStats<BinaryChromosome, MyPhenotype>>,
    prev_num_evaluations: u32,
}

impl Phenotype for MyPhenotype {}

impl fmt::Debug for MyPhenotype {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (&self.bit_grid as &dyn fmt::Display).fmt(f)
    }

}

impl Expressor<BinaryChromosome, MyPhenotype> for MyExpressor {
    fn express(&mut self, genotype: &BinaryChromosome) -> MyPhenotype {
        let mut bit_grid = BitGrid::new(SEED_PATCH_SIZE, SEED_PATCH_SIZE);
        let mut index = 0;

        for x in 0..SEED_PATCH_SIZE {
            for y in 0..SEED_PATCH_SIZE {
                if genotype.bits[index] {
                    bit_grid.set(x, y);
                }
                index += 1;
            }
        }

        MyPhenotype {
            bit_grid
        }
    }
}

impl fmt::Debug for MyExpressor {
    // Only show class name, not any state
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MyExpressor").finish()
    }
}

impl MyEvaluator {
    pub fn new() -> Self {
        MyEvaluator {
            gol: GameOfLife::new(GARDEN_SIZE, GARDEN_SIZE),
            gol_runner: GameOfLifeRunner::new(100, 2.0),
        }
    }
}

impl fmt::Debug for MyEvaluator {
    // Only show class name
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MyEvaluator").finish()
    }
}

impl Evaluator<MyPhenotype> for MyEvaluator {

    fn evaluate(&mut self, phenotype: &MyPhenotype) -> f32 {
        self.gol.reset();

        let xy0 = (GARDEN_SIZE - SEED_PATCH_SIZE) / 2;
        for x in 0..SEED_PATCH_SIZE {
            for y in 0..SEED_PATCH_SIZE {
                if phenotype.bit_grid.get(x, y) {
                    self.gol.set(xy0 + x, xy0 + y);
                }
            }
        }

        let stats = self.gol_runner.run(&mut self.gol);

        return (2 * stats.num_toggled - stats.ini_cells) as f32 + 1.0 / (stats.num_toggled_steps as f32 + 1.0);
    }
}

impl GenotypeFactory<BinaryChromosome> for MyConfig {
    fn create(&self) -> BinaryChromosome {
        BinaryChromosome::new(SEED_PATCH_SIZE * SEED_PATCH_SIZE)
    }
}

impl MyConfig {
    fn new() -> Self {
        MyConfig {
            mutation: BinaryBitMutation::new(1.0 / (SEED_PATCH_SIZE * SEED_PATCH_SIZE) as f32),
            recombination: BinaryNPointBitCrossover::new(1)
        }
    }
}

impl GenotypeManipulation<BinaryChromosome> for MyConfig {
    fn mutate(&self, target: &mut BinaryChromosome) {
        self.mutation.mutate(target);
    }

    fn recombine(&self, parent1: &BinaryChromosome, parent2: &BinaryChromosome) -> BinaryChromosome {
        self.recombination.recombine(parent1, parent2)
    }
}

impl GenotypeConfig<BinaryChromosome> for MyConfig {}

pub fn setup_ga() -> EvolutionaryAlgorithm<BinaryChromosome, MyPhenotype> {
    let ga_config = MyConfig::new();

    EvolutionaryAlgorithm::new(
        100,
        Box::new(ga_config),
        Box::new(MyExpressor {}),
        Box::new(MyEvaluator::new()),
        Box::new(TournamentSelection::new(3))
        // Box::new(
        //     ElitismSelection::new(
        //         1,
        //         Box::new(TournamentSelection::new(2))
        //     )
        // )
    ).set_mutation_prob(
        0.5
    ).set_recombination_prob(
        0.25
    ).enable_fitness_cache()
}

impl MyEvolutionaryAlgorithm {
    pub fn ea(&self) -> &EvolutionaryAlgorithm<BinaryChromosome, MyPhenotype> {
        &self.ea
    }
}

#[wasm_bindgen]
impl MyEvolutionaryAlgorithm {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        MyEvolutionaryAlgorithm {
            ea: setup_ga(),
            population_stats: None,
            prev_num_evaluations: 0,
        }
    }

    pub fn step(&mut self) {
        self.prev_num_evaluations = self.ea.num_evaluations();
        self.ea.step();
        self.population_stats = self.ea.get_population_stats();
    }

    pub fn num_generations(&self) -> u32 {
        self.ea.num_generations()
    }

    pub fn num_evaluations(&self) -> u32 {
        self.ea.num_evaluations()
    }

    pub fn evaluation_delta(&self) -> u32 {
        self.ea.num_evaluations() - self.prev_num_evaluations
    }

    pub fn max_fitness(&self) -> f32 {
        if let Some(stats) = &self.population_stats {
            stats.max_fitness
        } else {
            0.0
        }
    }

    pub fn avg_fitness(&self) -> f32 {
        if let Some(stats) = &self.population_stats {
            stats.avg_fitness
        } else {
            0.0
        }
    }

    pub fn best_phenotype(&self) -> String {
        if let Some(stats) = &self.population_stats {
            if let Some(phenotype) = &stats.best_indiv.phenotype() {
                return format!("{}", phenotype.bit_grid)
            }
        }
        String::from("None")
    }
}