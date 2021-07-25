//use wasm_bindgen::prelude::*;

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
};
use ga::binary::{
    BinaryChromosome,
    BinaryBitMutation,
    BinaryNPointBitCrossover
};
use ga::selection::{
    ElitismSelection,
    TournamentSelection,
};

const GARDEN_SIZE: usize = 64;
const SEED_PATCH_SIZE: usize = 8;

struct MyExpressor {}

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
                bit_grid.set(x, y, genotype.bits[index]);
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
    // Only show class name, not any state
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
                    self.gol.set(xy0 + x, xy0 + y, true);
                }
            }
        }

        let stats = self.gol_runner.run(&mut self.gol);

        return stats.max_cells as f32 + 1.0 / (stats.max_cells_steps as f32 + 1.0);
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
        Box::new(
            ElitismSelection::new(
                1,
                Box::new(TournamentSelection::new(2))
            )
        )
    ).set_mutation_prob(
        0.8
    ).set_recombination_prob(
        0.5
    )
}