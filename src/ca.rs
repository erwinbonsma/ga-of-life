use std::{clone, fmt};
use std::hash::{Hash};
use core::cmp::max;
use wasm_bindgen::prelude::*;

type UnitType = u64;
const BITS_PER_UNIT: usize = 64;

#[wasm_bindgen]
#[derive(clone::Clone, Debug, Hash, Eq, PartialEq)]
pub struct BitGrid {
    width: usize,
    height: usize,
    units_per_row: usize,
    units: Vec<UnitType>
}

pub struct BitCounter {
    lookup: Vec<u8>
}

const BITS_PER_UNIT_GOL: usize = BITS_PER_UNIT - 1;

#[derive(PartialEq)]
pub enum GridBorder {
    Zeroes,
    Wrapped
}

#[wasm_bindgen]
pub struct GameOfLife {
    bit_grid: BitGrid,
    width: usize,
    height: usize,
    border: GridBorder,
    units_per_row: usize,
    num_steps: u32,
    rows: [Vec<UnitType>; 3],
}

#[wasm_bindgen]
pub struct GameOfLifeRunner {
    // The minimum amount of absolute steps that the CA is allowed to be dormant, i.e. the total
    // number of steps that have passed since the maximum number of cells was reached. The CA will
    // never be terminated before this amount of passed.
    min_absolute_dormancy: u32,

    // The minimum relative number of steps that the CA is allowed to be dormant since the
    // maximum was reached. When this maximum was reached at time T, the CA will at least
    // run until time t >= T * (1 + min_relative_dormancy)
    min_relative_dormancy: f32,

    bit_counter: BitCounter,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct RunStats {
    // The number of cells initially alive
    pub ini_cells: u16,

    // The maximum number of cells that were alive at a given moment
    pub max_cells: u16,
    // The moment this occured
    pub max_cells_steps: u32,

    // The number of different cells that were alive at least once (at any time)
    pub num_toggled: u16,
    // The moment this was reached
    pub num_toggled_steps: u32,

    // The minimum number of cells that were alive since the maximum was reached
    pub min_cells_after_max: u16,
    // The moment this occured
    pub min_cells_after_max_steps: u32,

    // The total number of steps executed
    pub num_steps: u32
}

impl BitGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let extra_bits = width % BITS_PER_UNIT;
        let bits_per_row = if extra_bits > 0 { width + BITS_PER_UNIT - extra_bits } else { width };
        let units_per_row = bits_per_row / BITS_PER_UNIT;

        BitGrid {
            width,
            height,
            units_per_row,
            units: vec![0; (height * units_per_row) as usize]
        }
    }

    fn unit_index(&self, x: usize, y: usize) -> usize {
        (x / BITS_PER_UNIT) + self.units_per_row * y
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

    pub fn clear(&mut self, x: usize, y: usize) {
        let index = self.unit_index(x, y);
        let bitpos = x % BITS_PER_UNIT;

        self.units[index] = self.units[index] & !(1 << bitpos);
    }

    pub fn set(&mut self, x: usize, y: usize) {
        let index = self.unit_index(x, y);
        let bitpos = x % BITS_PER_UNIT;

        self.units[index] = self.units[index] | (1 << bitpos);
    }

    pub fn reset(&mut self) {
        self.units.iter_mut().for_each(|x| *x = 0);
    }

    pub fn toggle_all(&mut self) {
        self.units.iter_mut().for_each(|x| *x = !*x);
    }

    pub fn or(&mut self, other: &BitGrid) {
        if self.width == other.width && self.height == other.height {
            self.units.iter_mut().zip(other.units.iter()).for_each(|(x, y)| *x = *x | *y);
        } else {
            panic!("Not yet implemented");
        }
    }
}

impl fmt::Display for BitGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let symbol = if self.get(x, y) {
                    '◼'
                } else {
                    if (x % BITS_PER_UNIT) == BITS_PER_UNIT - 1 { '+' } else { '-' }
                };
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

    pub fn count_set_bits(&self, bit_grid: &BitGrid) -> usize {
        let mut count: usize = 0;
        for unit in bit_grid.units.iter() {
            count += self.lookup[(unit & 255) as usize] as usize;
            count += self.lookup[((unit >> 8) & 255) as usize] as usize;
            count += self.lookup[((unit >> 16) & 255) as usize] as usize;
            count += self.lookup[((unit >> 24) & 255) as usize] as usize;
            count += self.lookup[((unit >> 32) & 255) as usize] as usize;
            count += self.lookup[((unit >> 40) & 255) as usize] as usize;
            count += self.lookup[((unit >> 48) & 255) as usize] as usize;
            count += self.lookup[((unit >> 56) & 255) as usize] as usize;
        }
        count
    }

    // Counts the number of live cells in the provided bit grid, assuming the grid is storing the
    // state for the provided Game of Life configuration. Typically, the bit grid is the one owned
    // by the latter, but not necessarily. For example, for counting how many cells have toggled
    // state during a Game of Life run, a separate bit grid is maintained alongside the one that
    // represents the current state.
    fn count_live_cells_in_bitgrid(&self, gol: &GameOfLife, bit_grid: &BitGrid) -> usize {
        assert_eq!(gol.bit_grid.width, bit_grid.width);
        assert_eq!(gol.bit_grid.height, bit_grid.height);

        let mask_c: UnitType = !(1 << BITS_PER_UNIT_GOL);
        let mask_l = mask_c & !1;
        let mut mask_r = if gol.units_per_row == 1 {
            mask_l
        } else {
            mask_c
        };

        // The number of bits in the rightmost unit of each row that are used by the CA.
        let bits_in_last_unit = gol.width % BITS_PER_UNIT_GOL + 1;
        if bits_in_last_unit < BITS_PER_UNIT {
            mask_r &= !0 >> (BITS_PER_UNIT - bits_in_last_unit);
        }

        let mut count: usize = 0;
        let mut i = 0;
        for unit in bit_grid.units[
            gol.units_per_row..gol.units_per_row * (gol.height + 1)
        ].iter() {
            let mask = if i == gol.units_per_row - 1 {
                i = 0;
                mask_r
            } else if i == 0 {
                i = 1;
                mask_l
            } else {
                i += 1;
                mask_c
            };

            let val = *unit & mask;
            count += self.lookup[(val & 255) as usize] as usize;
            count += self.lookup[((val >> 8) & 255) as usize] as usize;
            count += self.lookup[((val >> 16) & 255) as usize] as usize;
            count += self.lookup[((val >> 24) & 255) as usize] as usize;
            count += self.lookup[((val >> 32) & 255) as usize] as usize;
            count += self.lookup[((val >> 40) & 255) as usize] as usize;
            count += self.lookup[((val >> 48) & 255) as usize] as usize;
            count += self.lookup[((val >> 56) & 255) as usize] as usize;
        }
        count
    }

    pub fn count_live_cells(&self, gol: &GameOfLife) -> usize {
        self.count_live_cells_in_bitgrid(gol, &gol.bit_grid)
    }
}


// Public implementation for GameOfLife that is excluded from WASM interface
impl GameOfLife {
    // The BitGrid used to represent the GameOfLife grid is larger than the latter. It is modified
    // as follows:
    // 1) There is an outside border of one cell around the entire grid. This speeds up computation
    //    as it means that branching can be avoided to handle calculations near the boundaries.
    // 2) Each unit in the GOL grid contains one fewer (effective) bit than the bit grid (i.e.
    //    BITS_PER_UNIT_GOL = BITS_PER_UNIT - 1). This is also done to speed up computation. It
    //    avoids the need to look the next unit column when updating cells _during_ the update
    //    loop.
    pub fn new_result(width: usize, height: usize, border: GridBorder) -> Result<Self, String> {
        let units_per_row = (width + 1 + (BITS_PER_UNIT_GOL - 1)) / BITS_PER_UNIT_GOL;

        if width < 3 || height < 3 {
            return Err("Size too small".to_string());
        }

        Ok(GameOfLife {
            bit_grid: BitGrid::new(units_per_row * BITS_PER_UNIT, height + 2),
            width,
            height,
            border,
            units_per_row,
            num_steps: 0,
            rows: [vec![0; units_per_row], vec![0; units_per_row], vec![0; units_per_row]]
        })
    }
}

#[wasm_bindgen]
impl GameOfLife {

    // The BitGrid used to represent the GameOfLife grid is larger than the latter. It is modified
    // as follows:
    // 1) There is an outside border of one cell around the entire grid. This speeds up computation
    //    as it means that branching can be avoided to handle calculations near the boundaries.
    // 2) Each unit in the GOL grid contains one fewer (effective) bit than the bit grid (i.e.
    //    BITS_PER_UNIT_GOL = BITS_PER_UNIT - 1). This is also done to speed up computation. It
    //    avoids the need to look the next unit column when updating cells _during_ the update
    //    loop.
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize, wrap_border: bool) -> Self {
        GameOfLife::new_result(
            width, height, if wrap_border {
                GridBorder::Wrapped
            } else {
                GridBorder::Zeroes
            }
        ).unwrap()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn num_cells(&self) -> usize {
        self.width * self.height
    }

    pub fn num_steps(&self) -> u32 {
        self.num_steps
    }

    pub fn reset(&mut self) {
        self.bit_grid.reset();
        self.num_steps = 0;
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

    fn set_wrapped_border(&mut self) {
        let units = &mut self.bit_grid.units;

        let mut unit_index_l = self.units_per_row;
        let mut unit_index_r = self.units_per_row * 2 - 1;
        let bit_pos_l_dst = 0;
        let bit_pos_l_src = 1;
        let bit_pos_r_dst = self.width % BITS_PER_UNIT_GOL + 1;
        let bit_pos_r_src = bit_pos_r_dst - 1;

        // Wrap left/right boundary columns
        for _ in 1..self.bit_grid.height - 1 {
            // Clear existing bit first
            units[unit_index_l] &= !(0x1 << bit_pos_l_dst);
            units[unit_index_r] &= !(0x1 << bit_pos_r_dst);

            // Copy wrapped bit
            units[unit_index_l] |= (units[unit_index_r] & (0x1 << bit_pos_r_src)) >> (bit_pos_r_src - bit_pos_l_dst);
            units[unit_index_r] |= (units[unit_index_l] & (0x1 << bit_pos_l_src)) << (bit_pos_r_dst - bit_pos_l_src);

            unit_index_l += self.units_per_row;
            unit_index_r += self.units_per_row;
        }

        // Wrap top/bottom boundary rows
        let (first_row, rest) = units.split_at_mut(self.units_per_row);
        let (body, last_row) = rest.split_at_mut(self.units_per_row * self.height);
        first_row.copy_from_slice(&body[self.units_per_row * (self.height - 1)..]);
        last_row.copy_from_slice(&body[..self.units_per_row]);
    }

    fn set_border_bits(&mut self) {
        match &self.border {
            GridBorder::Zeroes => self.set_zeroes_border(),
            GridBorder::Wrapped => self.set_wrapped_border()
        }
    }

    // Restore the rightmost bits of each unit. These will be incorrect after each update step, as
    // the update does not consider the neighbouring cell at their right (in the next unit).
    fn restore_right_bits(&mut self) {
        let units = &mut self.bit_grid.units;

        for unit_index in self.units_per_row..self.units_per_row * (self.height + 1) {
            // Clear incorrect value
            units[unit_index] &= !(0x1 << BITS_PER_UNIT_GOL);
            // Copy correct value from the next grid unit
            units[unit_index] |= (units[unit_index + 1] & 0x1) << BITS_PER_UNIT_GOL;
        }
    }

    // The Game of Life rules are implemented by performing bitwise calculations. This is based on
    // the Pico-8 implementation by rilden at: https://www.lexaloffle.com/bbs/?pid=94115
    pub fn step(&mut self) {
        let mut row_above = 0;
        let mut row_currn = 1;
        let mut row_below = 2;

        self.num_steps += 1;

        self.restore_right_bits();
        self.set_border_bits();

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

            // State needed for neighbours at the left (for leftmost cells in current unit column)
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
    }

    fn unit_index(&self, x: usize, y: usize) -> usize {
        (x + 1) / BITS_PER_UNIT_GOL + self.units_per_row * (y + 1)
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        let unit = self.bit_grid.units[self.unit_index(x, y)];
        let bitpos = (x + 1) % BITS_PER_UNIT_GOL;
        ((unit >> bitpos) & 1) == 1
    }

    pub fn clear(&mut self, x: usize, y: usize) {
        let index = self.unit_index(x, y);
        let bitpos = (x + 1) % BITS_PER_UNIT_GOL;
        self.bit_grid.units[index] &= !(1 << bitpos);
    }

    pub fn set(&mut self, x: usize, y: usize) {
        let index = self.unit_index(x, y);
        let bitpos = (x + 1) % BITS_PER_UNIT_GOL;
        self.bit_grid.units[index] |= 1 << bitpos;
    }
}

impl RunStats {
    pub fn new(ini_cells: u16) -> Self {
        RunStats {
            ini_cells,
            max_cells: ini_cells,
            max_cells_steps: 0,
            num_toggled: ini_cells,
            num_toggled_steps: 0,
            min_cells_after_max: ini_cells,
            min_cells_after_max_steps: 0,
            num_steps: 0
        }
    }
}

#[wasm_bindgen]
impl GameOfLifeRunner {

    #[wasm_bindgen(constructor)]
    pub fn new(min_absolute_dormancy: u32, min_relative_dormancy: f32) -> Self {
        GameOfLifeRunner {
            min_absolute_dormancy,
            min_relative_dormancy,
            bit_counter: BitCounter::new(),
        }
    }

    fn max_steps(&self, steps: u32) -> u32 {
        let rel_limit = steps + self.min_absolute_dormancy;
        let abs_limit = (steps as f32 * (1.0 + self.min_relative_dormancy)) as u32;

        max(rel_limit, abs_limit)
    }

    pub fn run(&self, gol: &mut GameOfLife) -> RunStats {
        let mut stats = RunStats::new(self.bit_counter.count_live_cells(gol) as u16);
        let mut max_steps = self.max_steps(0);
        let mut toggled = gol.bit_grid.clone();

        loop {
            gol.step();

            let mut dormant = true;
            let num_cells = self.bit_counter.count_live_cells(gol) as u16;

            if num_cells > stats.max_cells {
                stats.max_cells = num_cells;
                stats.max_cells_steps = gol.num_steps();
                stats.min_cells_after_max = num_cells;
                stats.min_cells_after_max_steps = stats.max_cells_steps;

                dormant = false;
            } else if num_cells < stats.min_cells_after_max {
                stats.min_cells_after_max = num_cells;
                stats.min_cells_after_max_steps = gol.num_steps();

                dormant = false;
            }

            toggled.or(&gol.bit_grid);
            let toggled_count = self.bit_counter.count_live_cells_in_bitgrid(gol, &toggled) as u16;
            if toggled_count > stats.num_toggled {
                stats.num_toggled = toggled_count;
                stats.num_toggled_steps = gol.num_steps();

                dormant = false;
            }

            if !dormant {
                max_steps = self.max_steps(gol.num_steps());
            } else if gol.num_steps() >= max_steps {
                stats.num_steps = gol.num_steps();
                return stats;
            }
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

        g.set(0, 0);
        g.set(15, 0);
        g.set(34, 0);
        g.set(57, 1);

        assert_eq!(bc.count_set_bits(&g), 4);
    }

    #[test]
    fn grid_invert() {
        let mut bit_grid = BitGrid::new(BITS_PER_UNIT, 3);
        let bc = BitCounter::new();

        bit_grid.toggle_all();
        assert_eq!(bc.count_set_bits(&bit_grid), BITS_PER_UNIT * 3);
    }

    mod game_of_life {
        use super::super::*;

        fn add_glider(gol: &mut GameOfLife, x: usize, y: usize) {
            // Glider pattern:
            //    *
            //      *
            //  * * *
            gol.set(1 + x, 0 + y);
            gol.set(2 + x, 1 + y);
            gol.set(0 + x, 2 + y);
            gol.set(1 + x, 2 + y);
            gol.set(2 + x, 2 + y);
        }

        fn add_leftwards_glider(gol: &mut GameOfLife, x: usize, y: usize) {
            // Glider pattern:
            //    *
            //  *
            //  * * *
            gol.set(1 + x, 0 + y);
            gol.set(0 + x, 1 + y);
            gol.set(0 + x, 2 + y);
            gol.set(1 + x, 2 + y);
            gol.set(2 + x, 2 + y);
        }

        fn add_blinker(gol: &mut GameOfLife, x: usize, y: usize) {
            // Blinker pattern:
            //   * * *
            gol.set(0 + x, y);
            gol.set(1 + x, y);
            gol.set(2 + x, y);
        }

        #[test]
        fn count_cells_all_ones() {
            let w = 58;
            let h = 3;
            let mut gol = GameOfLife::new(w, h, true);
            let bc = BitCounter::new();

            gol.bit_grid.toggle_all();

            assert_eq!(bc.count_live_cells(&gol), w * h);
        }

        #[test]
        fn grid_init() {
            let mut gol = GameOfLife::new(5, 5, true);
            let bc = BitCounter::new();

            gol.set(1, 2);
            gol.set(2, 2);
            gol.set(3, 2);

            assert_eq!(bc.count_set_bits(&gol.bit_grid), 3);
            assert_eq!(bc.count_live_cells(&gol), 3);
        }

        #[test]
        fn zeroes_border() {
            let w = 7;
            let h = 3;
            let mut gol = GameOfLife::new(w, h, false);
            let bc = BitCounter::new();

            gol.bit_grid.toggle_all();
            let num_bits = bc.count_set_bits(&gol.bit_grid);
            gol.set_border_bits();

            // All cells in actual grid should still be set
            assert_eq!(bc.count_live_cells(&gol), w * h);

            // At least all border cells should be cleared
            // Note: the implementation may clear more cells, outside the actual grid
            assert!(bc.count_set_bits(&gol.bit_grid) <= (num_bits - 2 * (w + h) - 4));
        }

        #[test]
        fn wrapped_border() {
            let w = 7;
            let h = 7;
            let mut gol = GameOfLife::new(w, h, true);

            gol.set(0, 0); // Corner
            gol.set(3, 0); // Top row
            gol.set(4, h - 1); // Bottom row
            gol.set(0, 2); // Left column
            gol.set(w - 1, 5); // Right column
            gol.set_border_bits();

            let bc = BitCounter::new();
            assert_eq!(bc.count_set_bits(&gol.bit_grid), 5 + 4 + 3);

            // Corner
            assert!(gol.bit_grid.get(1, h + 1));
            assert!(gol.bit_grid.get(w + 1, 1));
            assert!(gol.bit_grid.get(w + 1, h + 1));

            // Other points
            assert!(gol.bit_grid.get(4, h + 1));
            assert!(gol.bit_grid.get(5, 0));
            assert!(gol.bit_grid.get(w + 1, 3));
            assert!(gol.bit_grid.get(0, 6));
        }

        #[test]
        fn evolve_block() {
            let mut gol = GameOfLife::new(4, 4, true);
            let bc = BitCounter::new();

            // Pattern:
            //  * *
            //  * *
            gol.set(1, 1);
            gol.set(2, 1);
            gol.set(2, 1);
            gol.set(2, 2);

            gol.step();

            // Pattern should remain unchanged.
            assert_eq!(bc.count_live_cells(&gol), 4);
            assert!(gol.get(1, 1));
            assert!(gol.get(2, 1));
            assert!(gol.get(2, 1));
            assert!(gol.get(2, 2));
        }

        #[test]
        fn evolve_small_oscillator() {
            let mut gol = GameOfLife::new(5, 5, true);
            let bc = BitCounter::new();

            add_blinker(&mut gol, 1, 2);

            gol.step();

            // Pattern should have flipped to vertical orientation
            assert_eq!(bc.count_live_cells(&gol), 3);
            assert!(gol.get(2, 1));
            assert!(gol.get(2, 2));
            assert!(gol.get(2, 3));
        }

        #[test]
        fn evolve_glider() {
            let mut gol = GameOfLife::new(5, 5, true);
            let bc = BitCounter::new();

            add_glider(&mut gol, 1, 1);

            gol.step();
            gol.step();
            gol.step();
            gol.step();

            // Glider should have moved right and down one unit
            assert_eq!(bc.count_live_cells(&gol), 5);
            assert!(gol.get(3, 2));
            assert!(gol.get(4, 3));
            assert!(gol.get(2, 4));
            assert!(gol.get(3, 4));
            assert!(gol.get(4, 4));
        }

        #[test]
        fn evolve_toad_across_boundary() {
            let mut gol = GameOfLife::new(BITS_PER_UNIT * 2, 6, true);
            let bc = BitCounter::new();

            // Toad pattern:
            //    * * *
            //  * * *
            gol.set(BITS_PER_UNIT - 2, 2);
            gol.set(BITS_PER_UNIT - 1, 2);
            gol.set(BITS_PER_UNIT, 2);
            gol.set(BITS_PER_UNIT - 3, 3);
            gol.set(BITS_PER_UNIT - 2, 3);
            gol.set(BITS_PER_UNIT - 1, 3);

            gol.step();
            gol.step();

            // Toad should have osillated back to starting position
            assert_eq!(bc.count_live_cells(&gol), 6);
            assert!(gol.get(BITS_PER_UNIT - 2, 2));
            assert!(gol.get(BITS_PER_UNIT - 1, 2));
            assert!(gol.get(BITS_PER_UNIT, 2));
            assert!(gol.get(BITS_PER_UNIT - 3, 3));
            assert!(gol.get(BITS_PER_UNIT - 2, 3));
            assert!(gol.get(BITS_PER_UNIT - 1, 3));
        }

        #[test]
        fn evolve_glider_across_boundary() {
            let mut gol = GameOfLife::new(BITS_PER_UNIT * 2, 6, true);
            let bc = BitCounter::new();

            add_glider(&mut gol, BITS_PER_UNIT - 5, 0);

            for _ in 0..12 {
                gol.step();
                assert_eq!(bc.count_live_cells(&gol), 5);
            }

            // Glider should have moved across the boundary
            assert!(gol.get(BITS_PER_UNIT - 1, 3));
            assert!(gol.get(BITS_PER_UNIT, 4));
            assert!(gol.get(BITS_PER_UNIT - 2, 5));
            assert!(gol.get(BITS_PER_UNIT - 1, 5));
            assert!(gol.get(BITS_PER_UNIT, 5));
        }

        #[test]
        fn evolve_leftwards_glider_across_boundary() {
            let mut gol = GameOfLife::new(BITS_PER_UNIT * 2, 6, true);
            let bc = BitCounter::new();

            add_leftwards_glider(&mut gol, BITS_PER_UNIT, 0);

            for _ in 0..12 {
                gol.step();
                assert_eq!(bc.count_live_cells(&gol), 5);
            }

            // Glider should have moved across the boundary
            assert!(gol.get(BITS_PER_UNIT - 2, 3));
            assert!(gol.get(BITS_PER_UNIT - 3, 4));
            assert!(gol.get(BITS_PER_UNIT - 3, 5));
            assert!(gol.get(BITS_PER_UNIT - 2, 5));
            assert!(gol.get(BITS_PER_UNIT - 1, 5));
        }

        fn count_across_boundary(offset: usize) {
            let mut gol = GameOfLife::new(BITS_PER_UNIT + 10, 5, true);
            let bc = BitCounter::new();

            add_blinker(&mut gol, BITS_PER_UNIT - offset, 1);

            gol.step();

            assert_eq!(bc.count_live_cells(&gol), 3);
        }

        #[test]
        fn count_across_boundary_offset4() {
            count_across_boundary(4);
        }

        #[test]
        fn count_across_boundary_offset3() {
            count_across_boundary(3);
        }

        #[test]
        fn count_across_boundary_offset2() {
            count_across_boundary(2);
        }

        fn evolve_glider_across_wrapped_border(grid_size: usize) {
            let mut gol = GameOfLife::new(grid_size, grid_size, true);
            let bc = BitCounter::new();

            let x = 0;
            add_glider(&mut gol, x, 0);

            let num_steps = grid_size * 4;
            for _ in 0..num_steps {
                gol.step();
                assert_eq!(bc.count_live_cells(&gol), 5);
            }

            // Glider should have moved back to its starting position
            assert!(gol.get(x + 1, 0));
            assert!(gol.get(x + 2, 1));
            assert!(gol.get(x + 0, 2));
            assert!(gol.get(x + 1, 2));
            assert!(gol.get(x + 2, 2));
        }

        #[test]
        fn evolve_glider_across_wrapped_border_5x5() {
            evolve_glider_across_wrapped_border(5);
        }

        #[test]
        fn evolve_glider_across_wrapped_border_32x32() {
            evolve_glider_across_wrapped_border(32);
        }

        #[test]
        // Grid where a row of CA fits exactly in one grid unit
        fn evolve_glider_across_wrapped_border_62x62() {
            evolve_glider_across_wrapped_border(62);
        }

        #[test]
        // Grid where row just required two grid units
        fn evolve_glider_across_wrapped_border_63x63() {
            evolve_glider_across_wrapped_border(63);
        }

        #[test]
        fn evolve_glider_across_wrapped_border_64x64() {
            evolve_glider_across_wrapped_border(64);
        }

        fn evolve_glider_against_zeroes_border(w: usize) {
            let mut gol = GameOfLife::new(w, 8, false);
            let bc = BitCounter::new();

            add_glider(&mut gol, w - 5, 1);

            for _ in 0..30 {
                gol.step();
            }

            // The glider should become a block when reaching the zeroes border
            assert_eq!(bc.count_live_cells(&gol), 4);

            assert!(gol.get(w - 2, 5));
            assert!(gol.get(w - 1, 5));
            assert!(gol.get(w - 2, 6));
            assert!(gol.get(w - 1, 6));
        }

        #[test]
        fn evolve_glider_against_zeroes_border_w62() {
            evolve_glider_against_zeroes_border(62);
        }

        #[test]
        fn evolve_glider_against_zeroes_border_w63() {
            evolve_glider_against_zeroes_border(63);
        }

        #[test]
        fn evolve_glider_against_zeroes_border_w64() {
            evolve_glider_against_zeroes_border(64);
        }

        #[test]
        fn glider_termination() {
            let mut gol = GameOfLife::new(5, 5, true);
            let runner = GameOfLifeRunner::new(20, 2.0);

            add_glider(&mut gol, 1, 1);

            let stats = runner.run(&mut gol);

            assert_eq!(stats.max_cells, 5);
            assert_eq!(stats.max_cells_steps, 0);
        }

        #[test]
        fn penta_decathlon_termination() {
            let mut gol = GameOfLife::new(20, 15, true);
            let runner = GameOfLifeRunner::new(20, 2.0);

            for i in 5..15 {
                if i == 7 || i == 12 {
                    gol.set(6, i);
                    gol.set(8, i);
                } else {
                    gol.set(7, i);
                }
            }

            let stats = runner.run(&mut gol);

            assert_eq!(stats.max_cells, 40);
            assert!(stats.max_cells_steps < 15);
        }

        #[test]
        fn glider_toggled_count() {
            let size = 12;
            let mut gol = GameOfLife::new(size, size, true);
            let runner = GameOfLifeRunner::new(20, 2.0);

            add_glider(&mut gol, 1, 1);

            let stats = runner.run(&mut gol);

            assert!((stats.num_steps as usize) >= size * 4 * 2);
            assert!((stats.num_toggled as usize) == size * 4);
        }
    }
}