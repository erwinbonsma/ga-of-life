use super::{Genotype, Phenotype, Individual, Population, SelectionFactory, Selector};
use rand::{self, Rng};

#[derive(Clone, Copy, Debug)]
pub struct TournamentSelection {
    tournament_size: usize
}

struct TournamentSelector<P: Phenotype, G: Genotype<P>> {
    selection: TournamentSelection,
    population: Population<P, G>
}

impl TournamentSelection {
    pub fn new(tournament_size: usize) -> Self {
        TournamentSelection {
            tournament_size
        }
    }
}

impl<P: Phenotype, G: Genotype<P>> SelectionFactory<P, G> for TournamentSelection {
    fn select_from(&self, population: Population<P, G>) -> Box<dyn Selector<P, G>> {
        Box::new(
            TournamentSelector {
                selection: self.clone(),
                population
            }
        )
    }
}

impl<P: Phenotype, G: Genotype<P>> TournamentSelector<P, G> {
    fn select_one(&self) -> &Individual<P, G> {
        self.population.individuals.get(
            rand::thread_rng().gen_range(0..self.population.individuals.len())
        ).unwrap()
    }
}

impl<P: Phenotype, G: Genotype<P>> Selector<P, G> for TournamentSelector<P, G> {
    fn select(&self) -> &Individual<P, G> {
        let mut best = self.select_one();

        for _ in 1..self.selection.tournament_size {
            let other = self.select_one();

            if other.fitness > best.fitness {
                best = other;
            }
        }

        best
    }
}
