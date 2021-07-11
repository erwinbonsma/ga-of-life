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

const BITS_PER_UNIT_GOL: usize = BITS_PER_UNIT - 1;

pub enum GridBorder {
    Zeroes,
    Wraps
}

pub struct GameOfLife {
    bit_grid: BitGrid,
    width: usize,
    height: usize,
    border: GridBorder,
    units_per_row: usize,
    num_iterations: usize,
    rows: [Vec<u32>; 3],
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

    pub fn toggle_all(&mut self) {
        self.units.iter_mut().for_each(|x| *x = !*x);
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

    pub fn bits_in_game_of_life(&self, gol: &GameOfLife) -> usize {
        let mut count: usize = 0;
        let mut i = 0;
        for unit in gol.bit_grid.units[
            gol.units_per_row..gol.units_per_row * (gol.height + 1)
        ].iter() {
            let mut mask: u32 = !(1 << BITS_PER_UNIT_GOL);
            if i == 0 {
                mask &= !1;
            }
            if i == gol.units_per_row - 1 {
                mask &= !0 >> (BITS_PER_UNIT - ((gol.width + 1) % BITS_PER_UNIT_GOL));
                i = 0;
            } else {
                i += 1;
            }

            let val = *unit & mask;
            count += self.lookup[(val & 255) as usize] as usize;
            count += self.lookup[((val >> 8) & 255) as usize] as usize;
            count += self.lookup[((val >> 16) & 255) as usize] as usize;
            count += self.lookup[((val >> 24) & 255) as usize] as usize;
        }
        count
    }
}

impl GameOfLife {
    // The BitGrid used to represent the GameOfLife grid is larger than the latter. It is modified
    // as follows:
    // 1) There is an outside border of one cell around the entire grid. This speeds up computation
    //    as it means that branching can be avoided to handle calculations near the boundaries.
    // 2) Each unit in the GOL grid contains one fewer (effective) bit than the bit grid (i.e.
    //    BITS_PER_UNIT_GOL = BITS_PER_UNIT - 1). This is also done to speed up computation. It
    //    avoids the need to look the next unit column when updating cells _during_ the update
    //    loop.
    pub fn new(width: usize, height: usize, border: GridBorder) -> Self {
        let units_per_row = (width + 2 + (BITS_PER_UNIT_GOL - 1)) / BITS_PER_UNIT_GOL;

        GameOfLife {
            bit_grid: BitGrid::new(units_per_row * BITS_PER_UNIT, height + 2),
            width,
            height,
            border,
            units_per_row,
            num_iterations: 0,
            rows: [vec![0; units_per_row], vec![0; units_per_row], vec![0; units_per_row]]
        }
    }

    fn set_zeroes_border(&mut self) {
        let units = &mut self.bit_grid.units;
        units[
            0..self.units_per_row
        ].iter_mut().for_each(|x| *x = 0);

        let last_row_start = (self.bit_grid.height - 1) * self.units_per_row;
        units[
            last_row_start..last_row_start + self.units_per_row
        ].iter_mut().for_each(|x| *x = 0);

        let mut unit_index = self.units_per_row;
        let bit_mask_l = !0x1;
        let bit_mask_r = !(0x1 << ((self.width + 1) % BITS_PER_UNIT_GOL));
        for _ in 1..self.bit_grid.height - 1 {
            units[unit_index] &= bit_mask_l;
            unit_index += self.units_per_row - 1;
            units[unit_index] &= bit_mask_r;
            unit_index += 1;
        }
    }

    fn set_wrapping_border(&mut self) {
        // TODO
    }

    fn set_border_bits(&mut self) {
        match &self.border {
            GridBorder::Zeroes => self.set_zeroes_border(),
            GridBorder::Wraps => self.set_wrapping_border()
        }
    }

    fn restore_right_bits(&mut self) {
        let units = &mut self.bit_grid.units;

        for unit_index in self.units_per_row..self.units_per_row * (self.bit_grid.height + 1) {
            units[unit_index] &= !(0x1 << BITS_PER_UNIT_GOL);
            units[unit_index] |= (units[unit_index + 1] & 0x1) << BITS_PER_UNIT_GOL;
        }
    }

    pub fn step(&mut self) {
        let mut row_above = 0;
        let mut row_currn = 1;
        let mut row_below = 2;

        self.num_iterations += 1;

        // Init row above to Row #0 of grid
        self.rows[row_above][0..self.units_per_row].copy_from_slice(
            &self.bit_grid.units[0..self.units_per_row]
        );
        // Init current row to Row #1 of grid
        self.rows[row_currn][0..self.units_per_row].copy_from_slice(
            &self.bit_grid.units[self.units_per_row..self.units_per_row * 2]
        );

        let mut unit_index = self.units_per_row;
        for row in 1..self.bit_grid.height - 1 {
            // Init row below
            self.rows[row_below][0..self.units_per_row].copy_from_slice(
                &self.bit_grid.units[self.units_per_row * (row + 1)..self.units_per_row * (row + 2)]
            );

            // State needed for neighbours at the left (for rightmost cells in current unit column)
            let mut abc_sum_prev = 0;
            let mut abc_car_prev = 0;

            for col in 0..self.units_per_row {
                let above = self.rows[row_above][col];
                let below = self.rows[row_below][col];
                let currn = self.rows[row_currn][col];

                // above + below
                let ab_sum = above ^ below;
                let ab_car = above & below;

                // above + below + current
                let abc_sum = currn ^ ab_sum;
                let abc_car = currn & ab_sum | ab_car;

                // sum of bit0 (sum of sums)
                let l = abc_sum << 1 | abc_sum_prev >> (BITS_PER_UNIT_GOL - 1);
                let r = abc_sum >> 1; // Note: cannot include abc_sum_next, so incorrect for
                                      // rightmost bit.
                let lr = l ^ r;
                let sum0 = lr ^ ab_sum;
                let car0 = l & r | lr & ab_sum;

                // sum of bit1 (sum of carry's)
                let l = abc_car << 1 | abc_car_prev >> (BITS_PER_UNIT_GOL - 1);
                let r = abc_car >> 1;
                let lr = l ^ r;
                let sum1 = lr ^ ab_car;
                let car1 = l & r | lr & ab_car;

                self.bit_grid.units[unit_index] = (currn | sum0) & (car0 ^ sum1) & !car1;
                unit_index += 1;

                abc_sum_prev = abc_sum;
                abc_car_prev = abc_car;
            }

            let row_tmp = row_above;
            row_above = row_currn;
            row_currn = row_below;
            row_below = row_tmp;
        }

        self.restore_right_bits();
        self.set_border_bits();
    }

    fn unit_index(&self, x: usize, y: usize) -> usize {
        (x + 1) / BITS_PER_UNIT_GOL + self.units_per_row * (y + 1)
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        let unit = self.bit_grid.units[self.unit_index(x, y)];
        let bitpos = (x + 1) % BITS_PER_UNIT_GOL;
        ((unit >> bitpos) & 1) == 1
    }

    pub fn set(&mut self, x: usize, y: usize, val: bool) {
        let index = self.unit_index(x, y);
        let bitpos = (x + 1) % BITS_PER_UNIT_GOL;
        let units = &mut self.bit_grid.units;
        if val {
            units[index] = units[index] | (1 << bitpos);
        } else {
            units[index] = units[index] & !(1 << bitpos);
        }
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

    #[test]
    fn gol_all_ones_bit_count() {
        let w = 58;
        let h = 2;
        let mut gol = GameOfLife::new(w, h, GridBorder::Zeroes);
        let bc = BitCounter::new();

        gol.bit_grid.toggle_all();

        assert_eq!(bc.bits_in_game_of_life(&gol), w * h);
    }

    #[test]
    fn game_of_life_grid_init() {
        let mut gol = GameOfLife::new(5, 5, GridBorder::Zeroes);
        let bc = BitCounter::new();

        gol.set(1, 2, true);
        gol.set(2, 2, true);
        gol.set(3, 2, true);

        assert_eq!(bc.bits_in_grid(&gol.bit_grid), 3);
        assert_eq!(bc.bits_in_game_of_life(&gol), 3);
    }

    #[test]
    fn grid_invert() {
        let mut gol = GameOfLife::new(5, 5, GridBorder::Zeroes);
        let bc = BitCounter::new();

        gol.bit_grid.toggle_all();
        assert_eq!(bc.bits_in_grid(&gol.bit_grid), 7 * BITS_PER_UNIT);
    }

    #[test]
    fn zeroes_border() {
        let w = 7;
        let h = 3;
        let mut gol = GameOfLife::new(w, h, GridBorder::Zeroes);
        let bc = BitCounter::new();

        gol.bit_grid.toggle_all();
        let num_bits = bc.bits_in_grid(&gol.bit_grid);
        gol.set_border_bits();

        println!("{}", gol.bit_grid);
        
        // All cells in actual grid should still be set
        assert_eq!(bc.bits_in_game_of_life(&gol), w * h);

        // At least all border cells should be cleared
        // Note: the implementation may clear more cells, outside the actual grid
        assert!(bc.bits_in_grid(&gol.bit_grid) <= (num_bits - 2 * (w + h) - 4));
    }
}