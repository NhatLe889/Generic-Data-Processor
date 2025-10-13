use crate::traits::Processable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    pub fn new(name: &str, age: u32) -> Self {
        Self { 
            name: name.to_string(),
            age,
        }
    }
}

impl Processable for Person {
    fn key(&self) -> String {
        self.name.clone()
    }
}

impl Processable for i32 {
    fn key(&self) -> String {
        self.to_string()
    }
}

impl Processable for String {
    fn key(&self) -> String {
        self.clone()
    }
}