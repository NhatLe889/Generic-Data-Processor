mod traits;
mod data;
mod processor;

use crate::data::*;
use crate::processor::*;

fn main() {
    let numbers = vec![3, 2, 1];
    let words = vec!["apple".to_string(), "pear".to_string(), "banana".to_string()];
    let people = vec![
        Person::new("Kevin", 21),
        Person::new("John", 22),
        Person::new("Bobby", 32),
    ];

    //Using Generic Processor
    println!("Sorted numbers: {:?}", process_data(numbers));
    println!("Sorted words: {:?}", process_data(words));
    println!("Sorted people: {:?}", process_data(people));
}