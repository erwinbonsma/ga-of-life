use ga_of_life::{setup_ga};

fn main() {
    let mut ga = setup_ga();

    for i in 0..10 {
        ga.step();

        println!("{:?}", ga.get_stats());
        //println!("{:?}", ga);
    }
}
