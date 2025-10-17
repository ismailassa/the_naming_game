mod game_properties;
use crate::game_properties::utils::{Population, World};
use rand::Rng;

fn main() {
    println!("Start the Naming Game simulation!");
    let world = World::new(10);
    let population = Population::new(10);
    // println!("World: {:?}", world);
    // println!("Population: {:?}", population);

    // Context selection, consists of 5 objects
    let context_size = 5;
    let x = generate_random_numbers(context_size);
    println!("x: {:?}", x);
}

fn generate_random_numbers(amount: usize) -> Vec<u32> {
    let mut numbers = Vec::new();
    let mut rng = rand::rng();
    while numbers.len() < amount {
        let random_num = rng.random_range(0..10);
        if (numbers.contains(&random_num)) {
            continue;
        }
        numbers.push(random_num);
    }
    numbers
}
