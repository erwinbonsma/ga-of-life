use wasm_bindgen::prelude::*;
use console_error_panic_hook;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod ca;
pub mod ga;

use std::any::Any;
use std::fmt::{Debug, Display, Formatter, Result};
use ca::{BitGrid, GameOfLife, GameOfLifeRunner, RunStats};
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
    ElitismSelection,
    TournamentSelection,
};

const SEED_PATCH_SIZE: usize = 8;
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

trait FitnessCalculator {
    fn calculate_fitness(&self, run_stats: &RunStats) -> f32;
}

struct MyEvaluator {
    gol: GameOfLife,
    gol_runner: GameOfLifeRunner,
    num_ca_steps: u32,
    fitness_calculator: Box<dyn FitnessCalculator>,
}

#[derive(Debug)]
struct MyConfig {
    genotype_length: usize,
    mutation: BinaryBitMutation,
    recombination: BinaryUniformRecombination,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct FitnessWeights {
    pub num_toggled_cells: f32,
    pub num_toggled_steps: f32,
    pub max_alive_cells: f32,
    pub max_alive_steps: f32,
    pub num_start_cells: f32,
}

struct WeightedFitness {
    fitness_weights: FitnessWeights,
}

#[wasm_bindgen]
#[derive(Debug)]
// This struct contains the settings that can be modified by the user
pub struct MyEaSettings {
    // Problem settings
    garden_size: usize,
    wrap_border: bool,

    // Fitness weights
    fitness_weights: FitnessWeights,

    // Optimization settings
    mutation_rate: f32,
    recombination_rate: f32,
    population_size: usize,
    tournament_size: usize,
    elitism: bool,
}

#[wasm_bindgen]
pub struct MyEvolutionaryAlgorithm {
    ea: EvolutionaryAlgorithm<BinaryChromosome, MyPhenotype>,

    population_stats: Option<PopulationStats<BinaryChromosome, MyPhenotype>>,

    prev_num_evaluations: u32,
    prev_num_ca_steps: u32,

    gene_counts: Vec<u32>,
    gene_distribution: Vec<f32>,

    cell_counts: Vec<u32>,
    cell_distribution: Vec<f32>,
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
    pub fn new(
        garden_size: usize,
        wrap_border: bool,
        fitness_calculator: Box<dyn FitnessCalculator>
    ) -> Self {
        MyEvaluator {
            gol: GameOfLife::new(garden_size, garden_size, wrap_border),
            gol_runner: GameOfLifeRunner::new(100, 2.0),
            num_ca_steps: 0,
            fitness_calculator,
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

        let x0 = (self.gol.width() - SEED_PATCH_SIZE) / 2;
        let y0 = (self.gol.height() - SEED_PATCH_SIZE) / 2;
        for x in 0..SEED_PATCH_SIZE {
            for y in 0..SEED_PATCH_SIZE {
                if phenotype.bit_grid.get(x, y) {
                    self.gol.set(x0 + x, y0 + y);
                }
            }
        }

        let stats = self.gol_runner.run(&mut self.gol);
        self.num_ca_steps += stats.num_steps;

        self.fitness_calculator.calculate_fitness(&stats)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl WeightedFitness {
    fn new(fitness_weights: FitnessWeights) -> Self {
        WeightedFitness {
            fitness_weights: fitness_weights
        }
    }
}

impl FitnessCalculator for WeightedFitness {
    fn calculate_fitness(&self, stats: &RunStats) -> f32 {
        (stats.num_toggled as f32) * self.fitness_weights.num_toggled_cells
        + (stats.num_toggled_steps as f32) * self.fitness_weights.num_toggled_steps
        + (stats.max_cells as f32) * self.fitness_weights.max_alive_cells
        + (stats.max_cells_steps as f32) * self.fitness_weights.max_alive_steps
        + (stats.ini_cells as f32) * self.fitness_weights.num_start_cells
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

pub fn setup_ga(settings: &MyEaSettings) -> EvolutionaryAlgorithm<BinaryChromosome, MyPhenotype> {
    let expressor = MyNeutralExpressor::new(4);
    let main_selector = Box::new(TournamentSelection::new(
        settings.tournament_size
    ));

    EvolutionaryAlgorithm::new(
        settings.population_size,
        Box::new(MyConfig::new(expressor.genotype_length())),
        Box::new(expressor),
        Box::new(MyEvaluator::new(
            settings.garden_size,
            settings.wrap_border,
            Box::new(WeightedFitness::new(settings.fitness_weights))
        )),
        if settings.elitism {
            Box::new(ElitismSelection::new(1, main_selector))
        } else {
            main_selector
        }
    ).set_mutation_prob(
        settings.mutation_rate
    ).set_recombination_prob(
        settings.recombination_rate
    ).enable_fitness_cache()
}

impl MyEvolutionaryAlgorithm {
    pub fn ea(&self) -> &EvolutionaryAlgorithm<BinaryChromosome, MyPhenotype> {
        &self.ea
    }
}

#[wasm_bindgen]
impl FitnessWeights {
    pub fn new() -> Self {
        FitnessWeights {
            num_toggled_cells: 1.0,
            num_toggled_steps: 0.0,
            max_alive_cells: 0.0,
            max_alive_steps: 0.0,
            num_start_cells: 0.0,
        }
    }
}

#[wasm_bindgen]
impl MyEaSettings {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        MyEaSettings {
            garden_size: 64,
            wrap_border: false,
            fitness_weights: FitnessWeights::new(),
            mutation_rate: 0.9,
            recombination_rate: 0.4,
            population_size: 100,
            tournament_size: 2,
            elitism: true,
        }
    }

    pub fn set_garden_size(mut self, garden_size: usize) -> Self {
        self.garden_size = garden_size;
        self
    }
    pub fn garden_size(&self) -> usize {
        self.garden_size
    }

    pub fn set_border(mut self, wrap_border: bool) -> Self {
        self.wrap_border = wrap_border;
        self
    }
    pub fn border_wraps(&self) -> bool {
        self.wrap_border
    }

    pub fn set_fw_num_toggled_cells(mut self, value: f32) -> Self {
        self.fitness_weights.num_toggled_cells = value;
        self
    }
    pub fn set_fw_num_toggled_steps(mut self, value: f32) -> Self {
        self.fitness_weights.num_toggled_steps = value;
        self
    }
    pub fn set_fw_max_alive_cells(mut self, value: f32) -> Self {
        self.fitness_weights.max_alive_cells = value;
        self
    }
    pub fn set_fw_max_alive_steps(mut self, value: f32) -> Self {
        self.fitness_weights.max_alive_steps = value;
        self
    }
    pub fn set_fw_num_start_cells(mut self, value: f32) -> Self {
        self.fitness_weights.num_start_cells = value;
        self
    }

    pub fn set_mutation_rate(mut self, mutation_rate: f32) -> Self {
        self.mutation_rate = mutation_rate;
        self
    }
    pub fn mutation_rate(&self) -> f32 {
        self.mutation_rate
    }

    pub fn set_recombination_rate(mut self, recombination_rate: f32) -> Self {
        self.recombination_rate = recombination_rate;
        self
    }
    pub fn recombination_rate(&self) -> f32 {
        self.recombination_rate
    }

    pub fn set_population_size(mut self, size: usize) -> Self {
        self.population_size = size;
        self
    }
    pub fn population_size(&self) -> usize {
        self.population_size
    }

    pub fn set_tournament_size(mut self, size: usize) -> Self {
        self.tournament_size = size;
        self
    }
    pub fn tournament_size(&self) -> usize {
        self.tournament_size
    }

    pub fn set_elitism(mut self, elitism: bool) -> Self {
        self.elitism = elitism;
        self
    }
    pub fn elitism(&self) -> bool {
        self.elitism
    }
}

#[wasm_bindgen]
impl MyEvolutionaryAlgorithm {

    #[wasm_bindgen(constructor)]
    pub fn new(settings: &MyEaSettings) -> Self {
        console_error_panic_hook::set_once();

        MyEvolutionaryAlgorithm {
            ea: setup_ga(settings),
            population_stats: None,
            prev_num_evaluations: 0,
            prev_num_ca_steps: 0,
            gene_counts: vec![],
            gene_distribution: vec![],
            cell_counts: vec![],
            cell_distribution: vec![],
        }
    }

    pub fn reset(&mut self, settings: &MyEaSettings) {
        self.population_stats = None;
        self.prev_num_evaluations = 0;
        self.prev_num_ca_steps = 0;
        self.ea = setup_ga(settings);
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
                    .chars()
                    .filter(|ch| *ch != '\n')
                    .map(|ch| if ch == '◼' { '1' } else { '0' })
                    .collect::<String>()
            }
        }
        String::from("None")
    }

    pub fn best_genotype(&self) -> String {
        if let Some(stats) = &self.population_stats {
            stats.best_indiv.genotype().bits
                .iter()
                .map(|bit| if bit { '1' } else { '0' })
                .collect::<String>()
        } else {
            String::from("")
        }
    }

    pub fn gene_distribution(&mut self) -> *const f32 {
        self.gene_distribution.clear();
        let mut num_genotypes = 0;

        self.gene_counts.clear();
        self.gene_counts.extend(
            (0..self.ea.population().get_individual(0).genotype().bits.len())
                .map(|_| 0)
        );

        for indiv in self.ea.population().iter() {
            let genotype = indiv.genotype();
            num_genotypes += 1;

            for (index, gene) in genotype.bits.iter().enumerate() {
                if gene {
                    self.gene_counts[index] += 1;
                }
            }
        }

        self.gene_distribution.extend(
            self.gene_counts
                .iter()
                .map(|x| *x as f32 / num_genotypes as f32)
        );

        self.gene_distribution.as_ptr()
    }

    pub fn genotype_len(&self) -> u32 {
        self.gene_counts.len() as u32
    }

    pub fn cell_distribution(&mut self) -> *const f32 {
        self.cell_distribution.clear();
        let mut num_phenotypes = 0;

        self.cell_counts.clear();
        self.cell_counts.extend((0..TOTAL_SEED_CELLS).map(|_| 0));

        for indiv in self.ea.population().iter() {
            if let Some(phenotype) = indiv.phenotype() {
                let mut cell_index = 0;
                for y in 0..SEED_PATCH_SIZE {
                    for x in 0..SEED_PATCH_SIZE {
                        if phenotype.bit_grid.get(x, y) {
                            self.cell_counts[cell_index] += 1;
                        }
                        cell_index += 1;
                    }
                }
                num_phenotypes += 1;
            }
        }

        if num_phenotypes > 0 {
            self.cell_distribution.extend(
                self.cell_counts
                    .iter()
                    .map(|x| *x as f32 / num_phenotypes as f32)
            )
        }

        self.cell_distribution.as_ptr()
    }

    pub fn phenotype_len(&self) -> u32 {
        self.cell_counts.len() as u32
    }
}