use ga_of_life::{BitGrid};

fn main() {
    let mut bit_grid = BitGrid::new(32, 4);

    println!("{}", bit_grid);

    for i in 0..8 {
        let v = i * 7;
        let x = v % bit_grid.width();
        let y = v / bit_grid.width();
        bit_grid.set(x, y, true);
    }

    println!("{}", bit_grid);
}
