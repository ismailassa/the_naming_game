use std::ops::Index;

use fake::Fake;
use fake::faker::lorem::en::*;

#[derive(Debug)]
pub struct World {
    pub objects: Vec<u32>,
}

impl World {
    // I used the numbers as object identifiers for simplicity
    pub fn new(size: u32) -> Self {
        World {
            objects: (0..size).collect(),
        }
    }
}

#[derive(Debug)]
pub struct Population<T: Clone + PartialEq> {
    pub population: Vec<Agent<T>>,
}

impl<T: Clone + PartialEq> Population<T> {
    pub fn new(size: u32) -> Self {
        let mut population: Vec<Agent<T>> = Vec::new();
        for index in 0..size {
            let value = Agent {
                name: format!("agent_{}", index),
                vocabulary: Vocabulary { words: vec![] },
                role: Role::None,
            };
            population.push(value);
        }
        return Population { population };
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Agent<T: PartialEq> {
    pub name: String,
    pub vocabulary: Vocabulary<T>,
    pub role: Role,
}

impl<T: Clone + PartialEq> Agent<T> {
    pub fn create_word(&mut self, object: &T, score: f32) -> Word<T> {
        let fake_word: String = Word().fake();

        println!("Generated fake word: {}", fake_word);

        let word = Word {
            object: object.clone(),
            text: String::from(fake_word),
            score: score,
        };
        self.vocabulary.words.push(word.clone());

        word
    }

    pub fn has_word_for_object(&self, object: &T) -> bool {
        for word in &self.vocabulary.words {
            if (&word.object == object) {
                return true;
            }
        }
        false
    }

    pub fn get_common_word(&self, object: &T) -> Option<Word<T>> {
        let mut common_word: Option<Word<T>> = None;

        for word in &self.vocabulary.words {
            if &word.object == object {
                if common_word.is_none() {
                    common_word = Some(word.clone());
                    continue;
                };

                if word.score > common_word.as_ref().unwrap().score {
                    common_word = Some(word.clone());
                }
            }
        }

        return common_word;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    None = -1,
    Speaker = 0,
    Listener = 1,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vocabulary<T: PartialEq> {
    pub words: Vec<Word<T>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Word<T: PartialEq> {
    pub object: T,
    pub text: String,
    pub score: f32,
}
