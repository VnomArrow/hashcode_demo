use std::io::Write;
use rand::Rng;
use rand::prelude::ThreadRng;
use crate::{preferences::*, algorithms};
use crate::util::BaseTwoCounter;
use crate::algorithms::*;
use crate::util::*;

pub fn print_occurences(problem_name: &str) {
    let preference_info = parse_preference_info(("./src/inp/".to_string() + problem_name).as_str());
    let mut occurance_list = preference_info.list_of_occurences();
    let mut occurance_spectrum: Vec<usize> = vec!();
    occurance_spectrum.resize(occurance_list[0].count + 1, 0);
    occurance_list.reverse();
    for i in occurance_list {
        occurance_spectrum[i.count] += 1;
        println!("occurance: {} {}", i.count, preference_info.ingrediants[i.index].name);
    }
    for i in 0..occurance_spectrum.len() {
        println!("count {}: {}", i, occurance_spectrum[i]);
    }
}

pub fn find_by_occurances_and_walking(problem_name: &str) {
    // Get the 100 most common pizza
    let preference_info = parse_preference_info(("./src/inp/".to_string() + problem_name).as_str());
    let occurance_list = preference_info.list_of_occurences();
    let solver = CustomerCounter::new(&preference_info.persons, preference_info.ingrediants.len());
    let mut pizza_maker = PizzaMaker::new();

    // Create a subset where we only care about customers preferences of those ingredients
    let mut subset: Vec<usize> = vec!();
    for i in 0..1000 {
        subset.push(occurance_list[i].index);
    }

    let preference_subset = get_preference_info_from_subset(&subset, &preference_info);
    let solver_subset = CustomerCounter::new(&preference_subset.persons, preference_subset.ingrediants.len());

    let mut best_pizza = Pizza::empty_pizza(subset.len());
    let mut best_score = 0;
    let mut pow2_counter: crate::util::BaseTwoCounter = Default::default();
    for i in 0..100 {
        pow2_counter.inc();
        // Pick a random pizza
        let pizza = pizza_maker.random_pizza(preference_subset.ingrediants.len());
        let pizza = walk_first(&preference_subset, &pizza);
        let score = solver_subset.pizza_person_count(&pizza, preference_subset.persons.len());
        if score > best_score {
            best_pizza = pizza;
            best_score = score;
        }
    }

    println!("{:?}", best_pizza.ingrediants.len());

    for i in 0..20 {
        // Create a pizza using that info, and walk until we get the optimal pizza
        let random_pizza = pizza_maker.random_pizza_from_subset(&subset, &best_pizza, preference_info.ingrediants.len());
        let optimized_pizza = walk_first(&preference_info, &random_pizza);
        let best_score = solver.pizza_person_count(&optimized_pizza, preference_info.persons.len());
        optimized_pizza.save_to_file(best_score, ("./old/".to_string() + problem_name).as_str(), &preference_info.ingrediants);
    }
}

pub fn find_by_random(problem_name: &str) {
    // Take random ingrediants, it it is a new high score, save it
    // The case that gets the most users is the one you pick

    let preference_info = parse_preference_info(("./src/inp/".to_string() + problem_name).as_str());
    let solver = CustomerCounter::new(&preference_info.persons, preference_info.ingrediants.len());
    let mut pizza_maker = PizzaMaker::new();
    let mut iter_counter: BaseTwoCounter = Default::default();

    // Generate a random pizza
    let mut best_pizza = pizza_maker.random_pizza(preference_info.ingrediants.len());
    let mut best_score = solver.pizza_person_count(&best_pizza, preference_info.persons.len());
    loop {
        iter_counter.inc();
        // Generate a random pizza
        let pizza = pizza_maker.random_pizza(preference_info.ingrediants.len());
        let score = solver.pizza_person_count(&pizza, preference_info.persons.len());

        // If it has a new top score, save it to file
        if score > best_score {
            best_pizza = pizza;
            best_score = score;
            best_pizza.save_to_file(best_score, ("./old/".to_string() + problem_name).as_str(), &preference_info.ingrediants);
        }
    }
}

pub fn find_by_walking(problem_name: &str) {
    // Take a random pizza, then try and change each ingrediant, the one that gives the highest score is changed, then the process repeats

    let preference_info = parse_preference_info(("./src/inp/".to_string() + problem_name).as_str());
    let solver = CustomerCounter::new(&preference_info.persons, preference_info.ingrediants.len());
    let mut pizza_maker = PizzaMaker::new();
    let mut pizza = pizza_maker.random_pizza(preference_info.ingrediants.len());

    let optimized_pizza = algorithms::walk_steep(&preference_info, &pizza);
    let best_score = solver.pizza_person_count(&optimized_pizza, preference_info.persons.len());
    optimized_pizza.save_to_file(best_score, ("./old/".to_string() + problem_name).as_str(), &preference_info.ingrediants);
}

pub fn find_by_walking_v2(problem_name: &str) {
    // Take a random pizza, then try and change each ingrediant, the one that gives the highest score is changed, then the process repeats

    let preference_info = parse_preference_info(("./src/inp/".to_string() + problem_name).as_str());
    let solver = CustomerCounter::new(&preference_info.persons, preference_info.ingrediants.len());
    let mut pizza_maker = PizzaMaker::new();
    let mut pizza = pizza_maker.random_pizza(preference_info.ingrediants.len());

    let optimized_pizza = algorithms::walk_first(&preference_info, &pizza);
    let best_score = solver.pizza_person_count(&optimized_pizza, preference_info.persons.len());
    optimized_pizza.save_to_file(best_score, ("./old/".to_string() + problem_name).as_str(), &preference_info.ingrediants);
}

pub struct PizzaMaker {
    rng: ThreadRng
}
impl PizzaMaker {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng()
        }
    }
    pub fn random_pizza(&mut self, ingrediant_count: usize) -> Pizza {
        let mut ingrediants: Vec<bool> = vec!();
        ingrediants.resize(ingrediant_count, false);
        for i in 0..ingrediant_count {
            use rand::Rng;
            ingrediants[i] = self.rng.gen_bool(0.5f64);
        }
        Pizza { ingrediants }
    }
    pub fn random_pizza_from_subset(&mut self, subset: &Vec<usize>, old_pizza: &Pizza, ingrediant_count: usize) -> Pizza {
        let mut random_pizza = self.random_pizza(ingrediant_count);
        for i in 0..subset.len() {
            random_pizza.ingrediants[subset[i]] = old_pizza.ingrediants[i];
        }
        return random_pizza;
    }
}

#[derive(Debug, Clone)]
pub struct Pizza {
    pub ingrediants: Vec<bool>
}
impl Pizza {
    pub fn create_from_vec(ingrediant_count: usize, pizza: &Vec<usize>) -> Self {
        let mut ingrediants: Vec<bool> = vec!();
        ingrediants.resize(ingrediant_count, false);
        for i in pizza {
            ingrediants[*i] = true;
        }
        return Self {
            ingrediants
        };
    }
    pub fn empty_pizza(ingrediant_count: usize) -> Self {
        let mut ingrediants: Vec<bool> = vec!();
        ingrediants.resize(ingrediant_count, false);
        return Self {
            ingrediants
        };
    }
    // Saves the pizza in the speciefied folder with the score as the filename
    pub fn save_to_file(&self, score: usize, path: &str, ingredients: &Vec<Ingrediant>) {
        let mut name = String::from(path);
        name.push_str("/");
        name.push_str(&score.to_string());
        match std::fs::File::create(name) {
            Ok(mut file) => {
                let mut string = String::new();
                let mut added_ingredients_count = 0;
                for i in 0..self.ingrediants.len() {
                    if self.ingrediants[i] == true {
                        string.push_str(&ingredients[i].name);
                        string.push(' ');
                    }
                    added_ingredients_count += 1;
                }
                if let Err(err) = file.write_fmt(format_args!("{} {}", added_ingredients_count, string)) {
                    println!("Error {}", err);
                }
            }
            Err(err) => {
                println!("Error {}", err);
            }
        }

        println!("{} : {}", path, score)
    }
}

#[derive(Debug)]
pub struct CustomerCounter {
    ingrediants: Vec<Ingrediant>
}
impl CustomerCounter {
    pub fn new(list : &Vec<Person>, count: usize) -> Self {
        // Make ingrediants know which persons like and dislike it
        let mut ingrediants: Vec<Ingrediant> = vec!();
        ingrediants.resize_with(count, Default::default);
        for person in 0..list.len() {
            for liked in list[person].likes.iter() {
                ingrediants[*liked].liked_persons.push(person);
            }
            for disliked in list[person].dislikes.iter() {
                ingrediants[*disliked].disliked_persons.push(person);
            }
        }
        Self {
            ingrediants
        }
    }
    // How many people that can eat the pizza
    pub fn pizza_person_count(&self, pizza: &Pizza, people_count: usize) -> usize {
        // Create a list of all people and set them to true
        let mut people: Vec<bool> = Default::default();
        people.resize(people_count, true);

        // Set all people who cannot tolerate the pizza to false
        for i in 0..pizza.ingrediants.len() {
            if pizza.ingrediants[i] == true {
                // Remove all the people that dislike it
                for i in self.ingrediants[i].disliked_persons.iter() {
                    people[*i] = false;
                }
            }
            else {
                // Remove all people who liked it
                for i in self.ingrediants[i].liked_persons.iter() {
                    people[*i] = false;
                }
            }
        }

        // Count the people left and return the result
        let mut count = 0;
        for i in people {
            if i == true {
                count += 1;
            }
        }
        return count;
    }

    // How many people that can eat the pizza, taking into account ignored 
    pub fn pizza_person_count_ignored(&self, pizza: &Pizza, people_count: usize, ignored_ingrediants: &Pizza) -> usize {
        // Create a list of all people and set them to true
        let mut people: Vec<bool> = Default::default();
        people.resize(people_count, true);

        // Set all people who cannot tolerate the pizza to false
        for i in 0..pizza.ingrediants.len() {
            if ignored_ingrediants.ingrediants[i] == false {
                if pizza.ingrediants[i] == true {
                    // Remove all the people that dislike it
                    for i in self.ingrediants[i].disliked_persons.iter() {
                        people[*i] = false;
                    }
                }
                else {
                    // Remove all people who liked it
                    for i in self.ingrediants[i].liked_persons.iter() {
                        people[*i] = false;
                    }
                }
            }
        }

        // Count the people left and return the result
        let mut count = 0;
        for i in people {
            if i == true {
                count += 1;
            }
        }
        return count;
    }
}