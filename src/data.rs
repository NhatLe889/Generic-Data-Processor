use crate::traits::Processable;
use std::{cmp::Ordering, fmt::format};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

#[derive(Debug)]
pub struct Dog<'a>{
    pub owner: &'a Person,
    pub name: String
}

#[derive(Debug)]
pub enum MixedType {
    Integer(i32),
    Text(String),
    Identity(Person),
}

impl Person {
    pub fn new(name: &str, age: u32) -> Self {
        Self { 
            name: name.to_string(),
            age,
        }
    }
}

impl<'a> Dog<'a> {
    pub fn new(owner: &'a Person, name: &str) -> Self {
        Self {
            owner,
            name: name.to_string(),
        }
    }
}

impl<'a> Processable for Dog<'a> {
    fn key(&self) -> String {
        format!("{}_{}", self.name, self.owner.name)
    }
}

impl<'a> PartialEq for Dog<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl<'a> Eq for Dog<'a> {}

impl<'a> PartialOrd for Dog<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.key().cmp(&other.key()))
    }
}

impl<'a> Ord for Dog<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key().cmp(&other.key())
    }
}

impl Processable for MixedType {
    fn key(&self) -> String {
        match self {
            MixedType::Integer(i) => i.to_string(),
            MixedType::Text(s) => s.clone(),
            MixedType::Identity(p) => p.name.clone(),
        }
    }
}

impl PartialEq for MixedType {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl Eq for MixedType {}

impl PartialOrd for MixedType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.key().cmp(&other.key()))
    }
}

impl Ord for MixedType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key().cmp(&other.key())
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