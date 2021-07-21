use super::{Genotype, Phenotype, Individual, Population, Selection};
use rand::{self, Rng};
use std::cell::RefCell;

#[derive(Debug)]
pub struct TournamentSelection<P: Phenotype, G: Genotype<P>> {
    tournament_size: usize,

    population: Option<Population<P, G>>
}

#[derive(Debug)]
struct ElitismState<P: Phenotype, G: Genotype<P>> {
    num_selected_elites: usize,
    wrapped_selection: Box<dyn Selection<P, G>>,
}

#[derive(Debug)]
pub struct ElitismSelection<P: Phenotype, G: Genotype<P>> {
    // Configuration
    elite_size: usize,

    population: Option<Population<P, G>>,

    // Mutable state
    state: RefCell<ElitismState<P, G>>,
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

    fn select(&self) -> &Individual<P, G> {
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

impl<P: Phenotype, G: Genotype<P>> ElitismState<P, G> {
    pub fn new(wrapped_selection: Box<dyn Selection<P, G>>) -> Self {
        ElitismState {
            wrapped_selection,
            num_selected_elites: 0
        }
    }
}

impl<P: Phenotype, G: Genotype<P>>  ElitismSelection<P, G> {
    pub fn new(elite_size: usize, wrapped_selection: Box<dyn Selection<P, G>>) -> Self {
        ElitismSelection {
            elite_size,
            population: None,
            state: RefCell::new(ElitismState::new(wrapped_selection)),
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
        self.state.borrow_mut().num_selected_elites = 0;
    }

    fn select(&self) -> &Individual<P, G> {
        if self.state.borrow().num_selected_elites < self.elite_size {
            if let Some(population) = &self.population {
                let mut state = self.state.borrow_mut();
                let individual = population.individuals.get(state.num_selected_elites).unwrap();

                state.num_selected_elites += 1;

                if state.num_selected_elites == self.elite_size {
                    state.wrapped_selection.select_from( self.population.take().unwrap() );
                }
                
                individual
            } else {
                panic!("You must first invoke select_from");
            }
        } else {
            self.state.borrow().wrapped_selection.select()
        }
    }
}