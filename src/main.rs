mod game_properties;
use crate::game_properties::utils::{Agent, Population, Role, World};
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
    let mut agents: Vec<Agent<u32>> = get_random_elements(&population.population, 2);
    agents.get_mut(0).unwrap().role = Role::Speaker;
    agents.get_mut(1).unwrap().role = Role::Listener;
    println!("Context selected (object IDs): {:?}", context);
    println!("Agents selected: {:?}", agents);

    let mut rng = rand::rng();
    let random_index = rng.random_range(0..context_size);
    let topic_object = context[random_index];
    println!("Topic selected: {:?}", topic_object);

    agents.get_mut(0).unwrap().create_word(&topic_object, 0.4);

    agents.get_mut(0).unwrap().create_word(&topic_object, 0.6);

    let common_word = agents.get(0).unwrap().get_common_word(&topic_object);
    println!("Common word found: {:?}", common_word);
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
