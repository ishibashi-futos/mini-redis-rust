use crate_and_modules::{Fern, run_simulation};

fn main() {
    let mut fern = Fern {
        size: 1.0,
        growth_rate: 1.0001,
    };
    run_simulation(&mut fern, 1000);
    println!("final fern size: {}", fern.size);
}
