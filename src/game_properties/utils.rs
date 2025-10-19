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
pub struct Population<T> {
    pub population: Vec<Agent<T>>,
}

impl<T> Population<T> {
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
pub struct Agent<T> {
    pub name: String,
    pub vocabulary: Vocabulary<T>,
    pub role: Role,
}

impl<T: Clone> Agent<T> {
    pub fn create_word(&mut self, object: &T) -> Word<T> {
        let fake_word: String = Word().fake();

        println!("Generated fake word: {}", fake_word);

        let word = Word {
            object: object.clone(),
            text: String::from(fake_word),
            score: 0.5,
        };
        self.vocabulary.words.push(word.clone());

        word
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    None = -1,
    Speaker = 0,
    Listener = 1,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vocabulary<T> {
    pub words: Vec<Word<T>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Word<T> {
    pub object: T,
    pub text: String,
    pub score: f32,
}
