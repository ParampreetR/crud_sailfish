pub struct Db {
    pub persons: Vec<String>,
}

impl Db {
    pub fn new() -> Self {
        Self {
            persons: Vec::new(),
        }
    }

    pub fn add(&mut self, name: String) {
        self.persons.push(name);
    }

    pub fn delete(&mut self, index: usize) {
        self.persons.remove(index);
    }

    pub fn get(&self) -> Vec<String> {
        self.persons.clone()
    }
}
