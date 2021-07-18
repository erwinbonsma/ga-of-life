use super::{Genotype, Phenotype, Individual, Population, SelectionFactory, Selector};
use rand::{self, Rng};

#[derive(Clone, Copy, Debug)]
pub struct RankBasedSelection {
    group_size: usize
}

struct RankBasedSelector<P: Phenotype, G: Genotype<P>> {
    selection: RankBasedSelection,
    population: Population<P, G>
}

impl RankBasedSelection {
    pub fn new(group_size: usize) -> Self {
        RankBasedSelection {
            group_size
        }
    }
}

impl<P: Phenotype, G: Genotype<P>> SelectionFactory<P, G> for RankBasedSelection {
    fn select_from(&self, population: Population<P, G>) -> Box<dyn Selector<P, G>> {
        Box::new(
            RankBasedSelector {
                selection: self.clone(),
                population
            }
        )
    }
}

impl<P: Phenotype, G: Genotype<P>> RankBasedSelector<P, G> {
    fn select_one(&self) -> &Individual<P, G> {
        self.population.individuals.get(
            rand::thread_rng().gen_range(0..self.population.individuals.len())
        ).unwrap()
    }
}

impl<P: Phenotype, G: Genotype<P>> Selector<P, G> for RankBasedSelector<P, G> {
    fn select(&self) -> &Individual<P, G> {
        let mut best = self.select_one();

        for _ in 1..self.selection.group_size {
            let other = self.select_one();

            if other.fitness > best.fitness {
                best = other;
            }
        }

        best
    }
}
