use super::{Genotype, Phenotype, Individual, Population, Selection};
use rand::{self, Rng};
use std::iter::FromIterator;

#[derive(Debug)]
pub struct TournamentSelection {
    tournament_size: usize,
}

#[derive(Debug)]
pub struct ElitismSelection<P: Phenotype, G: Genotype<P>> {
    // Configuration
    elite_size: usize,
    wrapped_selection: Box<dyn Selection<P, G>>,

    // Mutable state
    ranking: Vec<usize>,
    num_selected_elites: usize,
}

impl TournamentSelection {
    pub fn new(tournament_size: usize) -> Self {
        TournamentSelection {
            tournament_size,
        }
    }

    fn select_one<'a, P: Phenotype, G: Genotype<P>>(
        &self, population: &'a Population<P, G>
    ) -> &'a Individual<P, G> {
        population.get_individual(
            rand::thread_rng().gen_range(0..population.size())
        )
    }
}

impl<P: Phenotype, G: Genotype<P>> Selection<P, G> for TournamentSelection {

    fn select_from<'a>(&mut self, population: &'a Population<P, G>) -> &'a Individual<P, G> {
        let mut best = self.select_one(population);

        for _ in 1..self.tournament_size {
            let other = self.select_one(population);

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
            ranking: Vec::new(),
            num_selected_elites: 0,
        }
    }
}

impl<P: Phenotype, G: Genotype<P>> Selection<P, G> for ElitismSelection<P, G> {
    fn start_selection(&mut self, population: &Population<P, G>) {
        // Sort individuals by fitness. Fittest first.
        if self.ranking.len() != population.size() {
            self.ranking = Vec::from_iter(0..population.size());
        }

        self.ranking.sort_by(
            |a, b| population.get_individual(*b).fitness.unwrap_or(0.0).partial_cmp(
                &population.get_individual(*a).fitness.unwrap_or(0.0)
            ).unwrap()
        );

        self.num_selected_elites = 0;

        self.wrapped_selection.start_selection(population);
    }

    fn preserve_next(&mut self) -> bool {
        self.num_selected_elites < self.elite_size
    }

    fn select_from<'a>(&mut self, population: &'a Population<P, G>) -> &'a Individual<P, G> {
        if self.num_selected_elites < self.elite_size {
            let individual = population.get_individual(
                *self.ranking.get(self.num_selected_elites).expect("Elite size exceeds ranking")
            );

            self.num_selected_elites += 1;

            individual
        } else {
            self.wrapped_selection.select_from(population)
        }
    }
}