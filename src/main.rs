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

    // 4. production
    let mut common_word = agents.get(0).unwrap().get_common_word(&topic_object);
    if common_word.is_none() {
        // 4a. invention
        common_word = Some(agents.get_mut(0).unwrap().create_word(&topic_object));
        agents[0].add_word(common_word.clone().unwrap());
    }
    let utter = common_word.clone().unwrap().text;
    println!("Utterance by agent 1: {:?}", utter);

    // 5. comprehension
    let pointing_object = agents.get(1).unwrap().get_word_by_text(&utter);
    println!("Pointing by agent 2: {:?}", pointing_object);

    if pointing_object.is_none() {
        println!("Agent 2 does not know the word '{}'", utter);
        println!("Communication failed!");
        return;
    }

    // Now calculate the success!
    if pointing_object.is_some() && pointing_object.as_ref().unwrap().object == topic_object {
        // in case topic and pointing.text match
        let pointing_word = pointing_object.unwrap();
        println!("Communication successful!");
        agents[0].update_score_sucessfull(&topic_object, pointing_word.clone());
        agents[1].update_score_sucessfull(&topic_object, pointing_word.clone());
    } else {
        let pointing_word = pointing_object.unwrap();
        println!(
            "Communication failed! Miss pointed to object {:?} but was {:?}",
            pointing_word.object,
            common_word.clone().unwrap().object
        );
        agents[0].update_score_failed_speaker(&topic_object, pointing_word.clone());
        agents[1].update_score_failed_listener(&utter);
        agents[1].add_word(common_word.unwrap());
    }
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
