use std::fmt;

const BITS_PER_UNIT: usize = 32;

pub struct BitGrid {
    width: usize,
    height: usize,
    units: Vec<u32>
}

pub struct BitCounter {
    lookup: Vec<u8>
}

impl BitGrid {
    pub fn new(width: usize, height: usize) -> Self {
        if width % BITS_PER_UNIT != 0 {
            panic!("Width should be a multiple of {}", BITS_PER_UNIT);
        }
        BitGrid {
            width,
            height,
            units: vec![0; height * width / BITS_PER_UNIT]
        }
    }

    fn unit_index(&self, x: usize, y: usize) -> usize {
        (x / BITS_PER_UNIT) + (self.width / BITS_PER_UNIT) * y
    }

    pub fn width(&self) -> usize { 
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        let unit = self.units[self.unit_index(x, y)];
        ((unit >> (x % BITS_PER_UNIT)) & 1) == 1
    }

    pub fn set(&mut self, x: usize, y: usize, val: bool) {
        let index = self.unit_index(x, y);
        let bitpos = x % BITS_PER_UNIT;
        if val {
            self.units[index] = self.units[index] | (1 << bitpos);
        } else {
            self.units[index] = self.units[index] & !(1 << bitpos);
        }
    }
}

impl fmt::Display for BitGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let symbol = if self.get(x, y) { '◼' } else { '◻' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl BitCounter {
    fn count_bits(mut val: u8) -> u8 {
        let mut count = 0;
        while val != 0 {
            if val & 1 == 1 {
                count += 1;
            }
            val >>= 1;
        }
        count
    }

    pub fn new() -> Self {
        let mut lookup = Vec::with_capacity(256);
        for i in 0..=255 {
            lookup.push(BitCounter::count_bits(i));
        }

        BitCounter {
            lookup
        }
    }

    pub fn bits_in_grid(&self, bit_grid: &BitGrid) -> usize {
        let mut count: usize = 0;
        for unit in bit_grid.units.iter() {
            count += self.lookup[(unit & 255) as usize] as usize;
            count += self.lookup[((unit >> 8) & 255) as usize] as usize;
            count += self.lookup[((unit >> 16) & 255) as usize] as usize;
            count += self.lookup[((unit >> 24) & 255) as usize] as usize;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_bit_count() {
        let mut g = BitGrid::new(BITS_PER_UNIT * 2, 2);
        let bc = BitCounter::new();

        g.set(0, 0, true);
        g.set(15, 0, true);
        g.set(34, 0, true);
        g.set(57, 1, true);

        assert_eq!(bc.bits_in_grid(&g), 4);
    }
}