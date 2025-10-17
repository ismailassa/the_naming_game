mod game_properties;
use crate::game_properties::utils::{Population, World};
use rand::Rng;

fn main() {
    println!("Start the Naming Game simulation!");
    let world_and_population_size: u32 = 10;
    let world = World::new(world_and_population_size);
    let population = Population::new(world_and_population_size);
    // println!("World: {:?}", world);
    // println!("Population: {:?}", population);

    // Context selection, consists of 5 objects
    let context_size: usize = 5;
    let context: Vec<u32> = get_random_elements(&world.objects, context_size);

    println!("Context selected (object IDs): {:?}", context);
}

fn get_random_elements<T: Clone + PartialEq>(elements: &Vec<T>, amount: usize) -> Vec<T> {
    let mut selected_elements = Vec::new();
    let mut rng = rand::rng();
    while selected_elements.len() < amount {
        let random_index = rng.random_range(0..elements.len());
        let element = elements[random_index].clone();
        if selected_elements.contains(&element) {
            continue;
        }
        selected_elements.push(element);
    }
    selected_elements
}
