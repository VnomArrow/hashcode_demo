use std::{fs::File, io::Write};

use rand::Rng;

use crate::vincent_code::{solve, Person};

pub fn create_random_pizza(max_ingredients: usize) -> Vec<usize> {
    let mut pizza = Vec::new();
    let mut rng = rand::thread_rng();

    for i in 0..max_ingredients {
        if rng.gen_bool(0.5f64) {
            pizza.push(i)
        }
    }
    pizza
}

pub fn print_pizza(problem: &str, score: usize, pizza: &Vec<usize>, names: &Vec<String>) {
    let mut name = String::from(problem);
    name.push_str("/");
    name.push_str(&score.to_string());
    match File::create(name) {
        Ok(mut file) => {
            let mut string = String::new();
            for i in 0..pizza.len() {
                string.push_str(&names[pizza[i]]);
                string.push(' ');
            }
            if let Err(err) = file.write_fmt(format_args!("{} {}", pizza.len(), string)) {
                println!("Error {}", err);
            }
        }
        Err(err) => {
            println!("Error {}", err);
        }
    }

    println!("{} : {}", problem, score)
}

// max_count: usize
pub fn run(problem: &str, persons: &Vec<Person>, max_ingredients: usize, names: &Vec<String>) {
    let mut best_pizza = create_random_pizza(max_ingredients);
    let mut best_random_score = solve(persons, &best_pizza);
    for _ in 0..10000 {
        let new_pizza = create_random_pizza(max_ingredients);
        let new_score = solve(persons, &new_pizza);
        if new_score > best_random_score {
            print_pizza(problem, new_score, &new_pizza, names);
            best_random_score = new_score;
            best_pizza = new_pizza;
        }
    }
    let mut best_score = best_random_score;
    loop {
        let current_loop_start_score = best_score;
        let pizza = best_pizza.clone();
        let mut all_ingredients = vec![true; max_ingredients];

        // try to remove all ingredients
        //(0..pizza.len()).into_par_iter().for_each(|i| {

        //});
        // try to remove all ingredients
        for i in 0..pizza.len() {
            let mut copy = pizza.clone();
            copy.remove(i);
            let score = solve(persons, &copy);
            if score > best_score {
                print_pizza(problem, best_score, &copy, names);
                best_score = score;
                best_pizza = copy;
            }
            all_ingredients[i] = false;
        }

        // try to add all ingredients
        for i in 0..max_ingredients {
            if all_ingredients[i] {
                let mut copy = pizza.clone();
                copy.push(i);
                let score = solve(persons, &copy);
                if score > best_score {
                    print_pizza(problem, best_score, &copy, names);
                    best_score = score;
                    best_pizza = copy;
                }
            }
        }
        print_pizza(problem, best_score, &best_pizza, names);
        if current_loop_start_score >= best_score {
            break;
        }
    }
}
