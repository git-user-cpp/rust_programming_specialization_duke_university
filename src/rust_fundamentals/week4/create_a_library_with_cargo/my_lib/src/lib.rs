#[derive(Debug)]
pub struct Person {
    name: String,
    age: i8,
}

impl Person {
    pub fn new(name: String, age: i8) -> Self {
        Self {
            name,
            age,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_age(&mut self, age: i8) {
        self.age = age;
    }

    pub fn get_age(&self) -> i8 {
        self.age
    }
}
