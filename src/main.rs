mod game_properties;
use crate::game_properties::utils::{Agent, Population, Role, World};
use rand::Rng;

fn main() {
    println!("Start the Naming Game simulation!");
    let world_and_population_size: u32 = 10;
    let world = World::new(world_and_population_size);
    let mut population = Population::new(world_and_population_size);
    // Context selection, consists of 5 objects
    let context_size: usize = 5;

    for _ in 0..100 {
        let context: Vec<u32> = get_random_elements(&world.objects, context_size);
        let agent_indices: Vec<usize> = get_random_indices(population.population.len(), 2);
        let speaker_idx = agent_indices[0];
        let listener_idx = agent_indices[1];
        population.population[speaker_idx].role = Role::Speaker;
        population.population[listener_idx].role = Role::Listener;

        println!("Context selected (object IDs): {:?}", context);

        let mut rng = rand::rng();
        let random_index = rng.random_range(0..context_size);
        let topic_object = context[random_index];
        println!("Topic selected: {:?}", topic_object);

        // 4. production
        let mut common_word = population.population[speaker_idx].get_common_word(&topic_object);
        if common_word.is_none() {
            // 4a. invention
            common_word = Some(population.population[speaker_idx].create_word(&topic_object));
            population.population[speaker_idx].add_word(common_word.clone().unwrap());
        }
        let utter = common_word.clone().unwrap().text;
        println!("Utterance by agent 1: {:?}", utter);

        // 5. comprehension
        let pointing_object = population.population[listener_idx].get_word_by_text(&utter);
        println!("Pointing by agent 2: {:?}", pointing_object);

        if pointing_object.is_none() {
            println!("Agent 2 does not know the word '{}'", utter);
            println!("Communication failed!");
            population.population[listener_idx].add_word(common_word.unwrap());
            continue;
        }

        // Now calculate the success!
        if pointing_object.is_some() && pointing_object.as_ref().unwrap().object == topic_object {
            // in case topic and pointing.text match
            let pointing_word = pointing_object.unwrap();
            println!("Communication successful!");
            population.population[speaker_idx]
                .update_score_sucessfull(&topic_object, pointing_word.clone());
            population.population[listener_idx]
                .update_score_sucessfull(&topic_object, pointing_word.clone());
        } else {
            let pointing_word = pointing_object.unwrap();
            println!(
                "Communication failed! Miss pointed to object {:?} but was {:?}",
                pointing_word.object,
                common_word.clone().unwrap().object
            );
            population.population[speaker_idx]
                .update_score_failed_speaker(&topic_object, pointing_word.clone());
            population.population[listener_idx].update_score_failed_listener(&utter);
            population.population[listener_idx].add_word(common_word.unwrap());
        }
    }

    println!("Agent states after the interaction:");
    println!("Populations: {:?}", population);
}

fn get_random_indices(max: usize, amount: usize) -> Vec<usize> {
    let mut selected_indices = Vec::new();
    let mut rng = rand::rng();
    while selected_indices.len() < amount {
        let random_index = rng.gen_range(0..max);
        if selected_indices.contains(&random_index) {
            continue;
        }
        selected_indices.push(random_index);
    }
    selected_indices
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
