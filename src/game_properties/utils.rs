pub struct World {
    pub objects: Vec<u32>,
}

impl World {
    pub fn new(&mut self, size: u32) {
        self.objects = (0..size).collect::<Vec<u32>>();
    }
}

pub struct Population {
    pub population: Vec<Agent>,
}

pub struct Agent {
    pub vocabulary: Vocabulary,
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
