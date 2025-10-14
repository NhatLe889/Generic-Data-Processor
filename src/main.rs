mod traits;
mod data;
mod processor;

use crate::data::*;
use crate::processor::*;

enum MixedType {
    Integer(i32),
    Text(String),
    Identity(Person),
}

fn main() {
    let numbers = vec![3, 2, 1, 5, 9, 18, 25];
    let words = vec!["apple".to_string(), "pear".to_string(), "banana".to_string(), "grape".to_string()];
    let people = vec![
        Person::new("Kevin", 21),
        Person::new("John", 22),
        Person::new("Bobby", 32),
        Person::new("Josh", 99),
        Person::new("George", 25),
    ];

    let unicorn_test = vec![
        MixedType::Integer(18),
        MixedType::Integer(3),
        MixedType::Identity(Person::new("Lebron", 40)),
        MixedType::Text("Strawberry".to_string()),
    ];
    //Implemnt sort for mixedtype later.

    //Using Generic Processor
    println!("Sorted numbers: {:?}", process_data(numbers));
    println!("Sorted words: {:?}", process_data(words));
    println!("Sorted people: {:?}", process_data(people));
}