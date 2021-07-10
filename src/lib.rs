use std::fmt;

const BITS_PER_UNIT: usize = 32;

pub struct BitGrid {
    width: usize,
    height: usize,
    units: Vec<u32>
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