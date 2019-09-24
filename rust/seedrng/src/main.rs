extern crate rand;
use rand::SeedableRng;
use rand::Rng;

fn main() {
    println!("Hello, world!");

    let mut rng: rand::rngs::StdRng = SeedableRng::from_seed([1; 32]);
    println!("{}", rng.gen::<f64>());

    let mut rng2: rand::rngs::StdRng = SeedableRng::from_seed([2; 32]);
    println!("{}", rng2.gen::<f64>());
}
