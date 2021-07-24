use std::{clone, fmt, slice};
use std::rc::Rc;
use rand::{self, Rng};

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
pub trait Phenotype : 'static + fmt::Debug {

}

/// A genotype encodes a solution to the optimisation problem.
pub trait Genotype<P: Phenotype> : 'static + fmt::Debug + clone::Clone {

    fn express(&self) -> P;

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

pub trait GenotypeFactory<P: Phenotype, G: Genotype<P>> {
    fn create(&self) -> G;
}

pub trait GenotypeManipulation<P: Phenotype, G: Genotype<P>> {
    fn mutate(&self, target: &mut G);
    fn recombine(&self, parent1: &G, parent2: &G) -> G;
}

pub trait GenotypeConfig<P: Phenotype, G: Genotype<P>>: 
    GenotypeFactory<P, G> + GenotypeManipulation<P, G> + fmt::Debug {}

#[derive(Debug)]
pub struct Individual<P: Phenotype, G: Genotype<P>> {
    genotype: Rc<G>,
    phenotype: Option<Rc<P>>,
    fitness: Option<f32>,
}

impl<P: Phenotype, G: Genotype<P>> Individual<P, G> {
    pub fn new(genotype: G) -> Self {
        Individual {
            genotype: Rc::new(genotype),
            phenotype: None,
            fitness: None
        }
    }
}

impl<P: Phenotype, G: Genotype<P>> clone::Clone for Individual<P, G> {
    fn clone(&self) -> Self {
        Individual {
            genotype: Rc::clone(&self.genotype),
            phenotype: match &self.phenotype {
                None => None,
                Some(phenotype) => Some(Rc::clone(phenotype))
            },
            fitness: self.fitness,
        }
    }
}

pub struct Population<P: Phenotype, G: Genotype<P>> {
    individuals: Vec<Individual<P, G>>,
}

impl<P: Phenotype, G: Genotype<P>> Population<P, G> {
    pub fn with_capacity(capacity: usize) -> Self {
        Population {
            individuals: Vec::with_capacity(capacity)
        }
    }

    pub fn get_individual(&self, index: usize) -> &Individual<P, G> {
        self.individuals.get(index).expect("Individual index out of range")
    }
 
    pub fn add_individual(&mut self, individual: Individual<P, G>) {
        self.individuals.push(individual);
    }

    pub fn size(&self) -> usize {
        self.individuals.len()
    }

    pub fn iter(&self) -> slice::Iter<'_, Individual<P, G>> {
        self.individuals.iter()
    }

    pub fn grow(&mut self) {
        // TODO: Check start state is "new_generation"
        for indiv in self.individuals.iter_mut() {
            if let None = indiv.phenotype {
                (*indiv).phenotype = Some(Rc::new(indiv.genotype.express()));
            }
        }
        // TODO: Update state to "grown"
    }

    pub fn evaluate(&mut self, evaluator: &mut(dyn Evaluator<P>)) {
        // TODO: Check start state is "grown"
        for indiv in self.individuals.iter_mut() {
            if let Some(phenotype) = &indiv.phenotype {
                if let None = indiv.fitness {
                    (*indiv).fitness = Some(evaluator.evaluate(&phenotype));
                }
            }
        }
        // TODO: Update state to "evaluated"
    }

    pub fn new_generation(&mut self, new_indivs: Vec<Individual<P, G>>) {
        // TODO: Check start state is "evaluated"
        assert_eq!(new_indivs.len(), self.size());

        self.individuals = new_indivs;
        // TODO: Update state to "new_generation"
    }
}

impl<P: Phenotype, G: Genotype<P>> fmt::Debug for Population<P, G> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for individual in self.individuals.iter() {
            write!(f, "\n{:?}", individual)?;
        }

        Ok(())
    }
}

pub trait Selection<P: Phenotype, G: Genotype<P>> : fmt::Debug {

    // Prepare new selection round, selecting from the given population.
    fn start_selection(&mut self, _population: &Population<P, G>) {
        // noop
    }

    // Returns "true" iff the next individual should be copied to the next generation unchanged.
    fn preserve_next(&mut self) -> bool {
        false
    }

    // Selects an individual.
    fn select_from<'a>(&mut self, population: &'a Population<P, G>) -> &'a Individual<P, G>;

}

#[derive(Debug)]
pub struct Stats {
    max_fitness: f32,
    avg_fitness: f32
}

#[derive(Debug)]
pub struct EvolutionaryAlgorithm<P: Phenotype, G: Genotype<P>> {
    pop_size: usize,
    recombination_prob: f32,
    mutation_prob: f32,
    evaluator: Box<dyn Evaluator<P>>,
    selection: Box<dyn Selection<P, G>>,
    config: Box<dyn GenotypeConfig<P, G>>,
    population: Population<P, G>,
}

impl<P: Phenotype, G: Genotype<P>> EvolutionaryAlgorithm<P, G> {
    pub fn new(
        pop_size: usize,
        config: Box<dyn GenotypeConfig<P, G>>,
        evaluator: Box<dyn Evaluator<P>>,
        selection: Box<dyn Selection<P, G>>
    ) -> Self {
        EvolutionaryAlgorithm {
            pop_size,
            config,
            recombination_prob: 0.5,
            mutation_prob: 0.8,
            evaluator,
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

    pub fn populate(&mut self) {
        while self.population.size() < self.pop_size {
            self.population.add_individual(
                Individual::new(self.config.create())
            );
        }
    }

    pub fn grow(&mut self) {
        self.population.grow();
    }

    pub fn evaluate(&mut self) {
        self.population.evaluate(&mut *(self.evaluator));
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

    fn next_individual(&mut self) -> Individual<P, G> {
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

    pub fn get_stats(&self) -> Option<Stats> {
        let mut max: Option<f32> = None;
        let mut sum: f32 = 0f32;
        let mut num: usize = 0;

        for individual in self.population.iter() {
            if let Some(fitness) = individual.fitness {
                sum += fitness;
                num += 1;
                max = Some(
                    match max {
                        None => fitness,
                        Some(current_max) => current_max.max(fitness)
                    }
                )
            }
        }

        if let Some(max_fitness) = max {
            let avg_fitness = sum / (num as f32);
            Some(Stats { max_fitness, avg_fitness })
        } else {
            None
        }
    }
}

pub mod selection;
pub mod binary;