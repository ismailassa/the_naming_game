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
pub struct Population {
    pub population: Vec<Agent>,
}

impl Population {
    pub fn new(size: u32) -> Self {
        let mut population: Vec<Agent> = Vec::new();
        for index in (0..size) {
            let mut value = Agent {
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
pub struct Agent {
    pub name: String,
    pub vocabulary: Vocabulary,
    pub role: Role,
}
impl Agent {}

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    None = -1,
    Speaker = 0,
    Listener = 1,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vocabulary {
    pub words: Vec<Word>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Word {
    pub object: u32,
    pub text: String,
    pub score: f32,
}
