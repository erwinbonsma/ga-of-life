use super::{Mutation, Recombination};
use bit_vec::BitVec;
use rand::{self, Rng};
use std::{clone, cmp};

#[derive(Debug)]
pub struct BinaryChromosome {
    pub bits: BitVec,
}

impl BinaryChromosome {
    pub fn new(size: usize) -> BinaryChromosome {
        let mut bits = BitVec::with_capacity(size);

        for _ in 0..size {
            bits.push(rand::random());
        }

        BinaryChromosome {
            bits
        }
    }

    pub fn zeroes(size: usize) -> BinaryChromosome {
        BinaryChromosome {
            bits: BitVec::from_elem(size, false)
        }
    }

    pub fn ones(size: usize) -> BinaryChromosome {
        BinaryChromosome {
            bits: BitVec::from_elem(size, true)
        }
    }
}

impl clone::Clone for BinaryChromosome {
    fn clone(&self) -> Self {
        BinaryChromosome {
            bits: self.bits.clone()
        }
    }
}

#[derive(Debug)]
pub struct BinaryBitMutation {
    mutate_prob: f32,
}

impl BinaryBitMutation {
    pub fn new(mutate_prob: f32) -> Self {
        BinaryBitMutation {
            mutate_prob
        }
    }
}

impl Mutation for BinaryBitMutation {
    type Genotype = BinaryChromosome;

    fn mutate(&self, target: &mut Self::Genotype) {
        // Instead of checking for each bit individually if it should be flipped, this function
        // calculates which bits should be flipped. It calculates which bit to mutate next as
        // follows:
        // 
        //   offset = floor( ln(1 - rnd_val) / ln(1 - p) )
        //
        // Here "rnd_val" is a random value in the range of [0, 1] and "p" is the probability of
        // mutating a bit. The "offset" is relative to the current bit.
        //
        // You can derive the above formula yourself from:
        //
        //   P(n <= N) = 1 - (1 - p)^N
        //
        // Where P(n <= N) is the probability that at least one of the "N" next bits changes.
        let denom = (1.0 - self.mutate_prob).ln();
        let mut i = 0;
        loop {
            let num = (1.0 - rand::thread_rng().gen::<f32>()).ln();

            // Note: the cast rounds towards zero and maps the infinity float value and other
            // values that are "too big" to the maximum integer value, which is what we want.
            i += (num / denom) as usize;
            if i >= target.bits.len() {
                return;
            }

            target.bits.set(i, !target.bits.get(i).unwrap());
            i += 1;
        }
    }
}

#[derive(Debug)]
pub struct BinaryNPointBitCrossover {
    n: usize,
}

impl BinaryNPointBitCrossover {
    pub fn new(n: usize) -> Self {
        BinaryNPointBitCrossover {
            n
        }
    }
}

impl Recombination for BinaryNPointBitCrossover {
    type Genotype = BinaryChromosome;

    fn recombine(
        &self, parent1: &Self::Genotype, parent2: &Self::Genotype
    ) -> Self::Genotype {

        let range = cmp::min(parent1.bits.len(), parent2.bits.len());
        let mut points: Vec<usize> = (0..self.n).map(
            |_| rand::thread_rng().gen_range(1..range)
        ).collect();
        &points[..].sort_unstable();

        if self.n % 2 == 1 {
            // Ensure that number of points is even
            points.push(parent1.bits.len());
        }

        let mut child = parent1.clone();
        for i in 0..points.len() / 2 {
            let from = points[i * 2];
            let to = points[i * 2 + 1];
            for j in from..to {
                child.bits.set(j, parent2.bits.get(j).unwrap());
            }
        }

        child
    }
}

#[derive(Debug)]
pub struct BinaryUniformRecombination {
    bias: f32,
}

impl BinaryUniformRecombination {

    /// Creates a new Binary Uniform Recombination operator.
    ///
    /// Bias should be in range [0, 1>. When it is 0 there is no bias. Bits are selected with
    /// equal probability from both parents. As bias increases, the one of the bits from one
    /// parent are increasingly favoured. As bias approaches 1, all bits are selected from one
    /// parent which means there is no recombination.
    pub fn new(bias: f32) -> Self {
        if bias < 0.0 || bias >= 1.0 {
            panic!("Bias out of range");
        }

        BinaryUniformRecombination {
            bias
        }
    }
}

impl Recombination for BinaryUniformRecombination {
    type Genotype = BinaryChromosome;

    fn recombine(
        &self, parent1: &Self::Genotype, parent2: &Self::Genotype
    ) -> Self::Genotype {
        let mut child = parent1.clone();
        let limit = 0.5 * (1.0 + self.bias);

        for i in 0..child.bits.len() {
            if rand::thread_rng().gen::<f32>() >= limit {
                child.bits.set(i, parent2.bits.get(i).unwrap());
            }
        }

        child
    }
}