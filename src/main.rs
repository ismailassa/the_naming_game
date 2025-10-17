mod game_properties;
use crate::game_properties::utils::{Population, World};

fn main() {
    println!("Start the Naming Game simulation!");
    let world = World::new(10);
    let population = Population::new(10);
    println!("World: {:?}", world);
    println!("Population: {:?}", population);
}
