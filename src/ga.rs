use std::{clone, fmt, slice};
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
    /// Evaluates the fitness for the phenotype
    ///
    /// TODO: Generalize to support cases where the fitness cannot be determined in isolation.
    /// E.g. where fitness is based on interaction with other individuals in the population.
    fn evaluate(&self) -> f32;
}

/// A genotype encodes a solution to the optimisation problem.
pub trait Genotype<P: Phenotype> : 'static + fmt::Debug + clone::Clone {

    fn express(&self) -> P;

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
    genotype: Box<G>,
    phenotype: Option<Box<P>>,
    fitness: Option<f32>,
}

impl<P: Phenotype, G: Genotype<P>> Individual<P, G> {
    pub fn new(genotype: Box<G>) -> Self {
        Individual {
            genotype,
            phenotype: None,
            fitness: None
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
 
    // TODO: Change GenotypeConfig to GenotypeFactory. This requires cast of trait to super trait.
    // See: https://users.rust-lang.org/t/casting-traitobject-to-super-trait/33524/8
    pub fn populate(&mut self, size: usize, genotype_factory: &(dyn GenotypeConfig<P, G>)) {
        while self.individuals.len() < size {
            self.individuals.push(
                Individual::new(Box::new(genotype_factory.create()))
            );
        }
    }

    pub fn add(&mut self, individual: Individual<P, G>) {
        self.individuals.push(individual);
    }

    pub fn size(&self) -> usize {
        self.individuals.len()
    }

    pub fn iter(&self) -> slice::Iter<'_, Individual<P, G>> {
        self.individuals.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, Individual<P, G>> {
        self.individuals.iter_mut()
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

pub trait Selector<P: Phenotype, G: Genotype<P>> {
    fn select(&self) -> &Individual<P, G>;
}

pub trait SelectionFactory<P: Phenotype, G: Genotype<P>>: fmt::Debug {
    fn select_from(&self, population: Population<P, G>) -> Box<dyn Selector<P, G>>;
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
    selection: Box<dyn SelectionFactory<P, G>>,
    config: Box<dyn GenotypeConfig<P, G>>,
    population: Option<Population<P, G>>,
}

impl<P: Phenotype, G: Genotype<P>> EvolutionaryAlgorithm<P, G> {
    pub fn new(
        pop_size: usize,
        config: Box<dyn GenotypeConfig<P, G>>,
        selection: Box<dyn SelectionFactory<P, G>>
    ) -> Self {
        EvolutionaryAlgorithm {
            pop_size,
            config,
            recombination_prob: 0.8,
            mutation_prob: 0.8,
            selection,
            population: None,
        }
    }

    pub fn start(&mut self) {
        let mut population = Population::with_capacity(self.pop_size);
        population.populate(self.pop_size, &*(self.config));

        self.population = Some(population);
    }

    pub fn grow(&mut self) {
        if let Some(population) = &mut self.population {
            for indiv in population.iter_mut() {
                if let None = indiv.phenotype {
                    (*indiv).phenotype = Some(Box::new(indiv.genotype.express()));
                }
            }
        }
    }

    pub fn evaluate(&mut self) {
        if let Some(population) = &mut self.population {
            for indiv in population.iter_mut() {
                if let Some(phenotype) = &indiv.phenotype {
                    if let None = indiv.fitness {
                        (*indiv).fitness = Some(phenotype.evaluate());
                    }
                }
            }
        }
    }

    /// Breeds a new generation of individuals. Their parents are selected from the current
    /// generation based on their fitness. The individuals will have a genotype, but their
    /// phenotype and fitness have not yet been determined. For this, use [grow] and [evaluate].
    pub fn breed(&mut self) {
        let old_population = self.population.take();
        let selector = (*self.selection).select_from(old_population.unwrap());
        let mut population = Population::with_capacity(self.pop_size);

        while population.size() < self.pop_size {
            let mut genotype = Box::new(
                if rand::thread_rng().gen::<f32>() < self.recombination_prob {
                    let parent1 = selector.select();
                    let parent2 = selector.select();
                    self.config.recombine(&parent1.genotype, &parent2.genotype)
                } else {
                    let parent = selector.select();
                    (*parent.genotype).clone()
                }
            );

            if rand::thread_rng().gen::<f32>() < self.mutation_prob {
                self.config.mutate(&mut genotype)
            }

            population.add(Individual::new(genotype))
        }

        self.population = Some(population);
    }

    pub fn get_stats(&self) -> Option<Stats> {
        if let Some(population) = &self.population {
            let mut max: Option<f32> = None;
            let mut sum: f32 = 0f32;
            let mut num: usize = 0;

            for individual in population.individuals.iter() {
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
        } else {
            None
        }
    }
}

pub mod selection;
pub mod binary;