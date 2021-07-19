use super::{Genotype, Phenotype, Individual, Population, Selection};
use rand::{self, Rng};

#[derive(Debug)]
pub struct TournamentSelection<P: Phenotype, G: Genotype<P>> {
    tournament_size: usize,
    population: Option<Population<P, G>>
}

impl<P: Phenotype, G: Genotype<P>> TournamentSelection<P, G> {
    pub fn new(tournament_size: usize) -> Self {
        TournamentSelection {
            tournament_size,
            population: None
        }
    }
}

impl<P: Phenotype, G: Genotype<P>> TournamentSelection<P, G> {
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
