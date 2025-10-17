pub struct World {
    pub objects: Vec<u32>,
}

impl World {
    // I used the numbers as object identifiers for simplicity
    pub fn new(&mut self, size: u32) {
        self.objects = (0..size).collect::<Vec<u32>>();
    }
}

pub struct Population {
    pub population: Vec<Agent>,
}

impl Population {
    pub fn new(&mut self, size: u32) {
        // self.population = (0..size)
        for _ in (0..size) {
            let value = Agent {
                vocabulary: Vocabulary { words: vec![] },
                role: Role::None,
            };
            self.population.push(value);
        }
    }
}

pub struct Agent {
    pub vocabulary: Vocabulary,
    pub role: Role,
}

pub enum Role {
    None = -1,
    Speaker = 0,
    Listener = 1,
}

impl Agent {}

pub struct Vocabulary {
    pub words: Vec<Word>,
}

pub struct Word {
    pub object: u32,
    pub text: String,
    pub score: f32,
}
