// Add parser
use crate::vincent_code::*;
use crate::lucas_code::*;

use std::{collections::HashMap};

#[allow(unused_macros)]
macro_rules! get_input {
    ($filename:expr) => {
        std::str::from_utf8(&std::fs::read($filename).unwrap())
        .unwrap()
        .lines()
    };
}

pub fn add_ingrediant(hashmap: &mut HashMap<String, usize>, counter: &mut usize, str: String) -> usize {
    if hashmap.contains_key(&str) {
        return hashmap[&str];
    }
    else {
        *counter += 1;
        hashmap.insert(str, *counter);
        return *counter;
    }
}

pub fn parser() -> Vec<Person> {
    let mut num = 0;
    let mut persons: Vec<Person> = Default::default();
    let mut current_person: Person = Default::default();
    let mut hashmap: HashMap<String, usize> = Default::default();
    let mut counter: usize = 0;
    for line in get_input!("./src/input.in") {
        if num == 0 {
            // Ignore first line
            num = 2;
            continue;
        }
        if num % 2 == 0 {
            // Things they like
            let mut line_split = line.split_whitespace();
            for i in line_split {
                current_person.likes.push(add_ingrediant(&mut hashmap, &mut counter, i.to_string()));
            }
        }
        if num % 2 == 1 {
            // Things they don't like
            let mut line_split = line.split_whitespace();
            for i in line_split {
                current_person.dislikes.push(add_ingrediant(&mut hashmap, &mut counter, i.to_string()));
            }
            persons.push(current_person);
            current_person = Default::default();
        }
        num += 1;
    }

    runner(&persons, counter);
    return persons;
}