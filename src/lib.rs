use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod ca;
pub mod ga;

use std::any::Any;
use std::fmt::{Debug, Display, Formatter, Result};
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
    BinaryUniformRecombination,
};
use ga::selection::{
    TournamentSelection,
};

const GARDEN_SIZE: usize = 64;
const SEED_PATCH_SIZE: usize = 8;
const TOTAL_GARDEN_CELLS: usize = GARDEN_SIZE * GARDEN_SIZE;
const TOTAL_SEED_CELLS: usize = SEED_PATCH_SIZE * SEED_PATCH_SIZE;

#[derive(Debug)]
struct MySimpleExpressor {}

#[derive(Debug)]
struct MyNeutralExpressor {
    bits_per_cell: u8,
    num_groups: u8,
    group_values: Vec<bool>,
}

#[derive(Hash, Eq, PartialEq)]
pub struct MyPhenotype {
    bit_grid: BitGrid,
}

struct MyEvaluator {
    gol: GameOfLife,
    gol_runner: GameOfLifeRunner,
    num_ca_steps: u32,
}

#[derive(Debug)]
struct MyConfig {
    genotype_length: usize,
    mutation: BinaryBitMutation,
    recombination: BinaryUniformRecombination,
}

#[wasm_bindgen]
pub struct MyEvolutionaryAlgorithm {
    ea: EvolutionaryAlgorithm<BinaryChromosome, MyPhenotype>,
    population_stats: Option<PopulationStats<BinaryChromosome, MyPhenotype>>,
    prev_num_evaluations: u32,
    prev_num_ca_steps: u32,
}

impl Phenotype for MyPhenotype {}

impl Debug for MyPhenotype {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        (&self.bit_grid as &dyn Display).fmt(f)
    }
}

impl Expressor<BinaryChromosome, MyPhenotype> for MySimpleExpressor {

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

impl MyNeutralExpressor {

    fn new(bits_per_cell: u8) -> Self {
        assert!(bits_per_cell > 1, "Neutral encoding should have more than one bit per cell");

        let num_groups = 0x1 << (bits_per_cell - 1);
        MyNeutralExpressor {
            bits_per_cell,
            num_groups,
            group_values: Vec::with_capacity(num_groups as usize),
        }
    }

    fn genotype_length(&self) -> usize {
        let num_seed_cells = SEED_PATCH_SIZE * SEED_PATCH_SIZE;

        num_seed_cells * self.bits_per_cell as usize + 2 * self.num_groups as usize
    }
}

impl Expressor<BinaryChromosome, MyPhenotype> for MyNeutralExpressor {

    fn express(&mut self, genotype: &BinaryChromosome) -> MyPhenotype {
        let mut bit_grid = BitGrid::new(SEED_PATCH_SIZE, SEED_PATCH_SIZE);

        // Determine values of each group
        let ng = self.num_groups as usize;
        self.group_values.clear();
        // Set to initial value
        for i in 0..ng {
            self.group_values.push(genotype.bits[i]);
        }
        // Optionally swap neighbours. This is done so that a mutation of a single "swap" bit can trigger an arbitrary
        // change to the phenotype. If two neighbouring groups have a different value, a swap means that one group of
        // alive cells is switched off and another set of dead cells (not necessarily of the same size) is turned on. 
        for i in 0..ng {
            if genotype.bits[ng + i] {
                // Swap with next
                let j = (i + 1) % ng;
                let tmp = self.group_values[i];
                self.group_values[i] = self.group_values[j];
                self.group_values[j] = tmp;
            }
        }

        // Skip first (num_groups * 2) bits as these are used for storing value of each group
        let mut index = 2 * ng;

        for y in 0..SEED_PATCH_SIZE {
            for x in 0..SEED_PATCH_SIZE {
                let mut n = (self.bits_per_cell - 1) as usize;
                let cell_state = if genotype.bits[index] {
                    // Use local majority voting
                    let mut votes = 0;

                    while n > 0 {
                        if genotype.bits[index + n] {
                            votes += 1;
                        }
                        n -= 1;
                    }

                    votes >= (self.bits_per_cell >> 1)
                } else {
                    // Take value from group
                    let mut group = 0;

                    while n > 0 {
                        group = group << 1;
                        if genotype.bits[index + n] {
                            group += 1;
                        }
                        n -= 1;
                    }

                    self.group_values[group]
                };

                if cell_state {
                    bit_grid.set(x, y);
                }

                index += self.bits_per_cell as usize;
            }
        }

        MyPhenotype {
            bit_grid
        }
    }
}

impl MyEvaluator {
    pub fn new() -> Self {
        MyEvaluator {
            gol: GameOfLife::new(GARDEN_SIZE, GARDEN_SIZE),
            gol_runner: GameOfLifeRunner::new(100, 2.0),
            num_ca_steps: 0,
        }
    }

    pub fn num_ca_steps(&self) -> u32 {
        self.num_ca_steps
    }
}

impl Debug for MyEvaluator {
    // Only show class name
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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

        self.num_ca_steps += stats.num_steps;

        (
            // Reward total garden cells toggled
            2 * (stats.num_toggled as i32 - TOTAL_GARDEN_CELLS as i32) +
            // Reward fewer seed cells
            TOTAL_SEED_CELLS as i32 - stats.ini_cells as i32
        ) as f32 
            // Reward quick coverage of garden
            //+ 1.0 / (stats.num_toggled_steps as f32 + 1.0)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl MyConfig {
    fn new(genotype_length: usize) -> Self {
        MyConfig {
            genotype_length,
            mutation: BinaryBitMutation::new(1.0 / genotype_length as f32),
            recombination: BinaryUniformRecombination::new(0.5)
        }
    }
}

impl GenotypeFactory<BinaryChromosome> for MyConfig {
    fn create(&self) -> BinaryChromosome {
        BinaryChromosome::new(self.genotype_length)
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
    let expressor = MyNeutralExpressor::new(4);

    EvolutionaryAlgorithm::new(
        100,
        Box::new(MyConfig::new(expressor.genotype_length())),
        Box::new(expressor),
        Box::new(MyEvaluator::new()),
        Box::new(TournamentSelection::new(3))
    ).set_mutation_prob(
        0.9
    ).set_recombination_prob(
        0.4
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
            prev_num_ca_steps: 0,
        }
    }

    pub fn step(&mut self) {
        self.prev_num_evaluations = self.ea.num_evaluations();
        self.prev_num_ca_steps = self.num_ca_steps();

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

    pub fn num_ca_steps(&self) -> u32 {
        match self.ea.evaluator().as_any().downcast_ref::<MyEvaluator>() {
            Some(my_evaluator) => my_evaluator.num_ca_steps(),
            None => panic!("Expected MyEvaluator as evaluator")
        }
    }

    pub fn ca_steps_delta(&self) -> u32 {
        self.num_ca_steps() - self.prev_num_ca_steps
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