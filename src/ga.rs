use std::{clone, fmt, slice};
use std::rc::Rc;
use std::ops::Deref;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use rand::{self, Rng};

/// A genotype encodes a solution to the optimisation problem.
pub trait Genotype : 'static + fmt::Debug + clone::Clone {
}

/// A phenotype represents a solution to the optimisation problem. How good the solution is is
/// expressed by its fitness, which influences selection by the evolutionary algorithm. 
///
/// A phenotype is an expression of a genotype. Multiple genotypes may result in the same
/// phenotype. Examples:
/// * A genotype can use a neutral encoding, i.e. an encoding with some redundancy. In this case,
///   the same solution can be expressed in multiple ways. Neutral encodings can help to prevent
///   the search from getting locked into a local optimum.
/// * For a given problem you may experiment with multiple genetic encodings, as the encoding
///   can have a big impact on the quality of the search. In this case, the phenotype remains the
///   same, as they all try to solve the same problem.
pub trait Phenotype : 'static + fmt::Debug + Hash + Eq {
}

pub trait Expressor<G: Genotype, P: Phenotype> : fmt::Debug {

    fn express(&mut self, genotype: &G) -> P;

}

pub trait Evaluator<P: Phenotype> : fmt::Debug {

    fn evaluate(&mut self, phenotype: &P) -> f32;

    // TODO: Extend with bulk_evaluate to support interaction-based fitness
}

pub trait Mutation {
    type Genotype;
    
    fn mutate(&self, target: &mut Self::Genotype);
}

pub trait Recombination {
    type Genotype;

    fn recombine(
        &self, parent1: &Self::Genotype, parent1: &Self::Genotype
    ) -> Self::Genotype;
}

pub trait GenotypeFactory<G: Genotype> {
    fn create(&self) -> G;
}

pub trait GenotypeManipulation<G: Genotype> {
    fn mutate(&self, target: &mut G);
    fn recombine(&self, parent1: &G, parent2: &G) -> G;
}

pub trait GenotypeConfig<G: Genotype>: 
    GenotypeFactory<G> + GenotypeManipulation<G> + fmt::Debug {}

#[derive(Debug)]
pub struct MyRef<T>(Rc<T>);

#[derive(Debug)]
pub struct Individual<G: Genotype, P: Phenotype> {
    genotype: MyRef<G>,
    phenotype: Option<MyRef<P>>,
    fitness: Option<f32>,
}

pub struct Population<G: Genotype, P: Phenotype> {
    individuals: Vec<Individual<G, P>>,
    fitness_cache: Option<HashMap<MyRef<P>, f32>>,
    generation: u32,
}

pub trait Selection<G: Genotype, P: Phenotype> : fmt::Debug {

    // Prepare new selection round, selecting from the given population.
    fn start_selection(&mut self, _population: &Population<G, P>) {
        // noop
    }

    // Returns "true" iff the next individual should be copied to the next generation unchanged.
    fn preserve_next(&mut self) -> bool {
        false
    }

    // Selects an individual.
    fn select_from<'a>(&mut self, population: &'a Population<G, P>) -> &'a Individual<G, P>;

}

#[derive(Debug)]
pub struct PopulationStats<G: Genotype, P: Phenotype> {
    pub max_fitness: f32,
    pub avg_fitness: f32,
    pub best_indiv: Individual<G, P>,
}

#[derive(Debug)]
pub struct CountingEvaluator<P: Phenotype> {
    evaluator: Box<dyn Evaluator<P>>,
    num_evaluations: u32,
}

#[derive(Debug)]
pub struct OptimizationStats {
    pub num_generations: u32,
    pub num_evaluations: u32,
}

#[derive(Debug)]
pub struct EvolutionaryAlgorithm<G: Genotype, P: Phenotype> {
    pop_size: usize,
    recombination_prob: f32,
    mutation_prob: f32,
    expressor: Box<dyn Expressor<G, P>>,
    evaluator: CountingEvaluator<P>,
    selection: Box<dyn Selection<G, P>>,
    config: Box<dyn GenotypeConfig<G>>,
    population: Population<G, P>,
}

impl<T> Deref for MyRef<T> {

    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }

}

impl<T: Hash> Hash for MyRef<T> {

    fn hash<H: Hasher>(&self, state: &mut H) {
        (*self.0).hash(state);
    }

}

impl<T: PartialEq> PartialEq for MyRef<T> {

    fn eq(&self, other: &Self) -> bool {
        *self.0 == *other.0
    }

}

impl<T: Eq> Eq for MyRef<T> {}

impl<T> MyRef<T> {

    pub fn new(wrapped: T) -> Self {
        MyRef(Rc::new(wrapped))
    }

    pub fn clone(&self) -> Self {
        MyRef(Rc::clone(&self.0))
    }

}

impl<G: Genotype, P: Phenotype> Individual<G, P> {
    pub fn new(genotype: G) -> Self {
        Individual {
            genotype: MyRef::new(genotype),
            phenotype: None,
            fitness: None
        }
    }

    pub fn phenotype(&self) -> Option<&P> {
        if let Some(phenotype) = &self.phenotype {
            Some(&(**phenotype))
        } else {
            None
        }
    }
}

impl<G: Genotype, P: Phenotype> clone::Clone for Individual<G, P> {
    fn clone(&self) -> Self {
        Individual {
            genotype: self.genotype.clone(),
            phenotype: match &self.phenotype {
                None => None,
                Some(phenotype) => Some(phenotype.clone())
            },
            fitness: self.fitness,
        }
    }
}

impl<G: Genotype, P: Phenotype> Population<G, P> {
    pub fn with_capacity(capacity: usize) -> Self {
        Population {
            individuals: Vec::with_capacity(capacity),
            fitness_cache: None,
            generation: 1,
        }
    }

    pub fn get_individual(&self, index: usize) -> &Individual<G, P> {
        self.individuals.get(index).expect("Individual index out of range")
    }
 
    pub fn add_individual(&mut self, individual: Individual<G, P>) {
        self.individuals.push(individual);
    }

    pub fn size(&self) -> usize {
        self.individuals.len()
    }

    pub fn iter(&self) -> slice::Iter<'_, Individual<G, P>> {
        self.individuals.iter()
    }

    pub fn grow(&mut self, expressor: &mut(dyn Expressor<G, P>)) {
        // TODO: Check start state is "new_generation"
        for indiv in self.individuals.iter_mut() {
            if let None = indiv.phenotype {
                (*indiv).phenotype = Some(
                    MyRef::new(expressor.express(&indiv.genotype))
                );
            }
        }
        // TODO: Update state to "grown"
    }

    pub fn evaluate(&mut self, evaluator: &mut(dyn Evaluator<P>)) {
        // TODO: Check start state is "grown"
        for indiv in self.individuals.iter_mut() {
            if let Some(phenotype) = &indiv.phenotype {
                if let None = indiv.fitness {
                    (*indiv).fitness = Some(
                        if let Some(cache) = &mut self.fitness_cache {
                            *cache.entry(phenotype.clone()).or_insert_with_key(
                                |phenotype| evaluator.evaluate(&phenotype)
                            )
                        } else {
                            evaluator.evaluate(&phenotype)
                        }
                    )
                }
            }
        }
        // TODO: Update state to "evaluated"
    }

    pub fn new_generation(&mut self, new_indivs: Vec<Individual<G, P>>) {
        // TODO: Check start state is "evaluated"
        assert_eq!(new_indivs.len(), self.size());

        self.individuals = new_indivs;
        self.generation += 1;
        // TODO: Update state to "new_generation"
    }

    pub fn get_stats(&self) -> Option<PopulationStats<G, P>> {
        let mut max: Option<f32> = None;
        let mut sum: f32 = 0f32;
        let mut num: usize = 0;
        let mut best_indiv = None;

        for individual in self.individuals.iter() {
            if let Some(fitness) = individual.fitness {
                sum += fitness;
                num += 1;
                if match max {
                    None => true,
                    Some(current_max) => fitness > current_max
                } {
                    max = Some(fitness);
                    best_indiv = Some(individual)
                }
            }
        }

        if let Some(max_fitness) = max {
            let avg_fitness = sum / (num as f32);
            Some(PopulationStats { 
                max_fitness,
                avg_fitness,
                best_indiv: (*best_indiv.unwrap()).clone()
            })
        } else {
            None
        }
    }
}

impl<G: Genotype, P: Phenotype> fmt::Debug for Population<G, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for individual in self.individuals.iter() {
            write!(f, "\n{:?}", individual)?;
        }

        Ok(())
    }
}

impl<P: Phenotype> CountingEvaluator<P> {
    pub fn new(
        evaluator: Box<dyn Evaluator<P>>,
    ) -> Self {
        CountingEvaluator {
            evaluator,
            num_evaluations: 0
        }
    }

    pub fn num_evaluations(&self) -> u32 {
        self.num_evaluations
    }
}

impl<P: Phenotype> Evaluator<P> for CountingEvaluator<P> {
    fn evaluate(&mut self, phenotype: &P) -> f32 {
        self.num_evaluations += 1;
        self.evaluator.evaluate(phenotype)
    }
}

impl<G: Genotype, P: Phenotype> EvolutionaryAlgorithm<G, P> {
    pub fn new(
        pop_size: usize,
        config: Box<dyn GenotypeConfig<G>>,
        expressor: Box<dyn Expressor<G, P>>,
        evaluator: Box<dyn Evaluator<P>>,
        selection: Box<dyn Selection<G, P>>
    ) -> Self {
        EvolutionaryAlgorithm {
            pop_size,
            config,
            recombination_prob: 0.5,
            mutation_prob: 0.8,
            expressor,
            evaluator: CountingEvaluator::new(evaluator),
            selection,
            population: Population::with_capacity(pop_size),
        }
    }

    pub fn set_recombination_prob(mut self, prob: f32) -> Self {
        self.recombination_prob = prob;
        self
    }

    pub fn set_mutation_prob(mut self, prob: f32) -> Self {
        self.mutation_prob = prob;
        self
    }

    pub fn enable_fitness_cache(mut self) -> Self {
        self.population.fitness_cache = Some(HashMap::new());
        self
    }

    pub fn evaluator(&self) -> &Box<dyn Evaluator<P>> {
        // Return wrapped evaluator to hide wrapping
        &self.evaluator.evaluator
    }

    pub fn num_generations(&self) -> u32 {
        self.population.generation
    }

    pub fn num_evaluations(&self) -> u32 {
        self.evaluator.num_evaluations
    }

    pub fn populate(&mut self) {
        while self.population.size() < self.pop_size {
            self.population.add_individual(
                Individual::new(self.config.create())
            );
        }
    }

    pub fn grow(&mut self) {
        self.population.grow(&mut *(self.expressor));
    }

    pub fn evaluate(&mut self) {
        self.population.evaluate(&mut self.evaluator);
    }

    fn new_genotype(&mut self) -> G {
        let mut genotype = if rand::thread_rng().gen::<f32>() < self.recombination_prob {
            let parent1 = (*self.selection).select_from(&self.population);
            let parent2 = (*self.selection).select_from(&self.population);
            self.config.recombine(&parent1.genotype, &parent2.genotype)
        } else {
            let parent = (*self.selection).select_from(&self.population);
            (*parent.genotype).clone()
        };

        if rand::thread_rng().gen::<f32>() < self.mutation_prob {
            self.config.mutate(&mut genotype)
        }

        genotype
    }

    fn next_individual(&mut self) -> Individual<G, P> {
        if (*self.selection).preserve_next() {
            (*(*self.selection).select_from(&self.population)).clone()
        } else {
            Individual::new(self.new_genotype())
        }
    }

    /// Breeds a new generation of individuals. Their parents are selected from the current
    /// generation based on their fitness. The individuals will have a genotype, but their
    /// phenotype and fitness have not yet been determined. For this, use [grow] and [evaluate].
    pub fn breed(&mut self) {
        let mut new_indivs = Vec::with_capacity(self.pop_size);

        (*self.selection).start_selection(&self.population);

        while new_indivs.len() < self.pop_size {
            new_indivs.push(self.next_individual());
        }

        self.population.new_generation(new_indivs);
    }

    pub fn step(&mut self) {
        if self.population.size() == 0 {
            self.populate();
        } else {
            self.breed();
        }

        self.grow();
        self.evaluate();
    }

    pub fn get_population_stats(&self) -> Option<PopulationStats<G, P>> {
        self.population.get_stats()
    }

    pub fn get_stats(&self) -> OptimizationStats {
        OptimizationStats {
            num_generations: self.population.generation,
            num_evaluations: self.evaluator.num_evaluations,
        }
    }
}

pub mod selection;
pub mod binary;