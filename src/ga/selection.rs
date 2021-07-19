use super::{Genotype, Phenotype, Individual, Population, Selection};
use rand::{self, Rng};

#[derive(Debug)]
pub struct TournamentSelection<P: Phenotype, G: Genotype<P>> {
    tournament_size: usize,
    population: Option<Population<P, G>>
}

#[derive(Debug)]
pub struct ElitismSelection<P: Phenotype, G: Genotype<P>> {
    // Configuration
    elite_size: usize,
    wrapped_selection: Box<dyn Selection<P, G>>,

    // Selection state
    population: Option<Population<P, G>>,
    num_selected_elites: usize,
}

impl<P: Phenotype, G: Genotype<P>> TournamentSelection<P, G> {
    pub fn new(tournament_size: usize) -> Self {
        TournamentSelection {
            tournament_size,
            population: None
        }
    }

    fn select_one(&self) -> &Individual<P, G> {
        if let Some(population) = &self.population {
            population.individuals.get(
                rand::thread_rng().gen_range(0..population.individuals.len())
            ).unwrap()
        } else {
            panic!("You must first invoke select_from");
        }
    }
}

impl<P: Phenotype, G: Genotype<P>> Selection<P, G> for TournamentSelection<P, G> {
    fn select_from(&mut self, population: Population<P, G>) {
        self.population = Some(population);
    }

    fn select(&mut self) -> &Individual<P, G> {
        let mut best = self.select_one();

        for _ in 1..self.tournament_size {
            let other = self.select_one();

            if other.fitness > best.fitness {
                best = other;
            }
        }

        best
    }
}

impl<P: Phenotype, G: Genotype<P>>  ElitismSelection<P, G> {
    pub fn new(elite_size: usize, wrapped_selection: Box<dyn Selection<P, G>>) -> Self {
        ElitismSelection {
            elite_size,
            wrapped_selection,
            population: None,
            num_selected_elites: 0
        }
    }
}

impl<P: Phenotype, G: Genotype<P>> Selection<P, G> for ElitismSelection<P, G> {
    fn select_from(&mut self, population: Population<P, G>) {
        // Sort individuals by fitness. Fittest first.
        let mut pop = population;
        pop.individuals.sort_by(
            |a, b| b.fitness.unwrap_or(0.0).partial_cmp(
                &a.fitness.unwrap_or(0.0)
            ).unwrap()
        );

        self.population = Some(pop);
        self.num_selected_elites = 0;
    }

    fn select(&mut self) -> &Individual<P, G> {
        if self.num_selected_elites < self.elite_size {
            if let Some(population) = &self.population {
                let individual = population.individuals.get(self.num_selected_elites).unwrap();

                self.num_selected_elites += 1;

                if self.num_selected_elites == self.elite_size {
                    self.wrapped_selection.select_from( self.population.take().unwrap() );
                }
        
                individual
            } else {
                panic!("You must first invoke select_from");
            }
        } else {
            self.wrapped_selection.select()
        }
    }
}