// Add parser

use std::collections::HashMap;

use crate::vincent_code::{Person, solve, alternate_solve};

#[allow(unused_macros)]
macro_rules! get_input {
    ($filename:expr) => {
        std::str::from_utf8(&std::fs::read($filename).unwrap()).unwrap()
    };
}

pub fn add_ingrediant(
    hashmap: &mut HashMap<String, usize>,
    counter: &mut usize,
    names: &mut Vec<String>,
    str: String,
) -> usize {
    if hashmap.contains_key(&str) {
        return hashmap[&str];
    } else {
        *counter += 1;
        names.push(str.clone());
        hashmap.insert(str, *counter - 1);

        return *counter - 1;
    }
}

fn read<T>(in_string: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    in_string.parse::<T>().unwrap()
}

pub fn parser(file: &str) -> (Vec<Person>, usize, Vec<String>) {
    let mut persons: Vec<Person> = Default::default();

    let mut hashmap: HashMap<String, usize> = Default::default();
    let mut counter: usize = 0;
    let mut names: Vec<String> = Vec::new();

    let data = &std::fs::read(file).unwrap();
    let mut lines = std::str::from_utf8(data).unwrap().lines();

    let ppl = read::<usize>(lines.next().unwrap());
    for _ in 0..ppl {
        let mut current_person: Person = Default::default();

        let like = lines.next().unwrap();
        let mut line_split = like.split_whitespace();
        line_split.next();

        for i in line_split {
            current_person
                .likes
                .push(add_ingrediant(&mut hashmap, &mut counter, &mut names, i.to_string()));
        }
        let dislike = lines.next().unwrap();

        let mut line_split = dislike.split_whitespace();
        line_split.next();
        for i in line_split {
            current_person
                .dislikes
                .push(add_ingrediant(&mut hashmap, &mut counter, &mut names, i.to_string()));
        }
        persons.push(current_person);
    }

    return (persons, counter, names);
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Default)]
pub struct Ingrediant {
    pub name: String,
    pub occurance_count: usize,
    pub liked_persons: Vec<usize>,
    pub disliked_persons: Vec<usize>,
}

pub fn get_digit_number(num: usize, digits: usize) -> String {
    let num_str = num.to_string();
    let mut add_str = String::new();
    for i in 0..(5 - num_str.len()) {
        add_str += "0";
    }
    return add_str + num_str.as_str();
}

#[derive(Debug, Default)]
pub struct PizzaPreferenceInfo {
    pub persons: Vec<Person>,
    pub ingrediant_name_to_index_hashmap: HashMap<String, usize>, // Returns the index in the ingrediants
    pub ingrediants: Vec<Ingrediant>
}


#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Default)]
pub struct Occurance {
    pub count: usize,
    pub index: usize
}
impl PizzaPreferenceInfo {
    // Adds occurance for ingrediant and makes sure it exists
    // Returns the ingrediant index
    fn add_ingrediant_occurance(&mut self, ingrediant_name: &str) -> usize {
        // Try to fetch the ingrediants index
        let ingrediant_index_option = self.ingrediant_name_to_index_hashmap.get(ingrediant_name);
        let ingrediant_index: usize;
        if ingrediant_index_option.is_some() {
            // The ingrediant exists
            ingrediant_index = *ingrediant_index_option.unwrap();
        }
        else {
            // The ingreadiant does not exist yet and must be created
            ingrediant_index = self.ingrediants.len();

            self.ingrediants.push(Ingrediant {
                name: ingrediant_name.to_string(),
                occurance_count: 0,
                liked_persons: vec!(),
                disliked_persons: vec!()
            });

            self.ingrediant_name_to_index_hashmap.insert(ingrediant_name.to_string(), ingrediant_index);
        }

        // Add an occurance for ingrediant
        self.ingrediants[ingrediant_index].occurance_count += 1;
        return ingrediant_index;
    }
    pub fn add_liked_ingrediant(&mut self, ingrediant_name: &str, person_index: usize) {
        let ingrediant_index = self.add_ingrediant_occurance(ingrediant_name);

        // Make ingrediant know that the person likes it
        self.ingrediants[ingrediant_index].liked_persons.push(person_index);

        // Make person know it likes the ingrediant
        self.persons[person_index].likes.push(ingrediant_index)
    }

    pub fn add_disliked_ingrediant(&mut self, ingrediant_name: &str, person_index: usize) {
        let ingrediant_index = self.add_ingrediant_occurance(ingrediant_name);
        
        // Make ingrediant know that the person dislikes it
        self.ingrediants[ingrediant_index].disliked_persons.push(person_index);

        // Make person know it dislikes the ingrediant
        self.persons[person_index].dislikes.push(ingrediant_index)
    }
    pub fn list_of_occurences(&self) -> Vec<Occurance> {
        // Crate the list and list all of the ingredients and their occurance
        let mut occurance_list: Vec<Occurance> = vec!();
        occurance_list.reserve(self.ingrediants.len());
        for i in 0..self.ingrediants.len() {
            occurance_list.push(Occurance{count: self.ingrediants[i].occurance_count, index: i})
        }

        // Sort the list by occurances, biggest first
        occurance_list.sort_by(|a, b| b.count.cmp(&a.count));

        return occurance_list;
    }
}
// Code to calculate occurances of ingredients
pub fn parse_preference_info(file: &str) -> PizzaPreferenceInfo {
    let mut preference_info: PizzaPreferenceInfo = Default::default();

    let data = &std::fs::read(file).unwrap();
    let mut lines = std::str::from_utf8(data).unwrap().lines();

    let ppl = read::<usize>(lines.next().unwrap());
    for _ in 0..ppl {
        preference_info.persons.push(Default::default());

        let like = lines.next().unwrap();
        let mut line_split = like.split_whitespace();
        line_split.next();

        for i in line_split {
            preference_info.add_liked_ingrediant(i, preference_info.persons.len() - 1);
        }
        let dislike = lines.next().unwrap();

        let mut line_split = dislike.split_whitespace();
        line_split.next();
        for i in line_split {
            preference_info.add_disliked_ingrediant(i, preference_info.persons.len() - 1);
        }
    }


    return preference_info;


    /*let mut hashmap: HashMap<String, usize> = Default::default();
    let mut counter: usize = 0;
    let mut names: Vec<String> = Vec::new();

    let data = &std::fs::read("./src/inp/d_difficult").unwrap();
    let mut lines = std::str::from_utf8(data).unwrap().lines();

    let ppl = read::<usize>(lines.next().unwrap());
    for _ in 0..ppl {
        let like = lines.next().unwrap();
        let mut line_split = like.split_whitespace();
        line_split.next();

        for i in line_split {

            let a = add_ingrediant(&mut hashmap, &mut counter, &mut names, i.to_string());
            if a >= occurance_count.len() {
                occurance_count.resize(a+1, 0);
            }
            occurance_count[a] += 1;
        }
        let dislike = lines.next().unwrap();

        let mut line_split = dislike.split_whitespace();
        line_split.next();
        for i in line_split {
            let a = add_ingrediant(&mut hashmap, &mut counter, &mut names, i.to_string());
            if a >= occurance_count.len() {
                occurance_count.resize(a+1, 0);
            }
            occurance_count[a] += 1;
        }
    }
    let mut occurances = vec!();
    for i in 0..occurance_count.len() {
        occurances.push(Occurance{name: names[i].clone(), count: occurance_count[i]});
    }
    occurances.sort_by(|a, b| b.count.cmp(&a.count));
    for i in 0..occurance_count.len() {
        println!("{} {}", occurances[i].count, occurances[i].name);
    }
    return occurances;*/
}

pub fn example_new_old() {
    use alternate_solve::*;
    use crate::runner::*;
    let (persons, counter, names) = parser("./src/inp/d_difficult");
    let pizza = create_random_pizza(counter);
    let score = solve(&persons, &pizza);
    println!("old score {}", score);

    let mut preference_info = parse_preference_info("./src/inp/d_difficult");
    let solver = alternate_solve::Solver::new(&preference_info.persons, preference_info.ingrediants.len());
    let pizza = Pizza::create_from_vec(preference_info.ingrediants.len(), &pizza);
    let mut best_score = solver.pizza_person_count(&pizza, preference_info.persons.len());
    println!("new score {}", best_score);
}

pub fn victor_generate_pizza() {
    // Start by taking the most influential ingerdient and randomizing a bunch of pizzas with or without it
    // The case that gets the most users is the one you pick
    use alternate_solve::*;
    use crate::runner::*;

    let preference_info = parse_preference_info("./src/inp/d_difficult");
    let solver = alternate_solve::Solver::new(&preference_info.persons, preference_info.ingrediants.len());
    let mut pizza_maker = PizzaMaker::new();
    let mut best_pizza = pizza_maker.random_pizza(preference_info.ingrediants.len());
    let mut best_score = solver.pizza_person_count(&best_pizza, preference_info.persons.len());
    best_pizza.save_to_file(best_score, "./old/d_difficult", &preference_info.ingrediants);
    /*let occurance_list = preference_info.list_of_occurences();
    for i in occurance_list {
        println!("occurance: {} {}", i.count, preference_info.ingrediants[i.index].name);
    }*/
    let mut iter_count = 0;
    let mut base_2 = 1;
    loop {
        iter_count += 1;
        if iter_count >= base_2 {
            println!("count {}", iter_count);
            base_2 *= 2;
        }
        // Generate a random pizza
        let pizza = pizza_maker.random_pizza(preference_info.ingrediants.len());
        let score = solver.pizza_person_count(&pizza, preference_info.persons.len());

        // If it has a new top score, save it to file
        if score > best_score {
            best_pizza = pizza;
            best_score = score;
            best_pizza.save_to_file(best_score, "./old/d_difficult", &preference_info.ingrediants);
        }
    }
}

pub fn find_by_walking() {
    use alternate_solve::*;
    use crate::runner::*;

    let preference_info = parse_preference_info("./src/inp/d_difficult");
    let solver = alternate_solve::Solver::new(&preference_info.persons, preference_info.ingrediants.len());
    let mut pizza_maker = PizzaMaker::new();
    let mut pizza = pizza_maker.random_pizza(preference_info.ingrediants.len());

    println!("customer count: {}", preference_info.persons.len());

    loop {
        // Set the best pizza to the previous pizza
        let mut best_pizza = pizza.clone();
        let mut best_score = solver.pizza_person_count(&best_pizza, preference_info.persons.len());

        let mut changed = false;
        // Try changing the ingrediants one by one and pick the one that gives the best pizza
        for i in 0..preference_info.ingrediants.len() {
            let mut pizza_copy = pizza.clone();
            pizza_copy.ingrediants[i] = !pizza_copy.ingrediants[i];
            let score = solver.pizza_person_count(&pizza_copy, preference_info.persons.len());
            if score > best_score {
                best_pizza = pizza_copy;
                best_score = score;
                changed = true;
            }
        }

        if changed {
            pizza = best_pizza;
        }
        else {
            break;
        }
    }

    let best_score = solver.pizza_person_count(&pizza, preference_info.persons.len());
    pizza.save_to_file(best_score, "./old/d_difficult", &preference_info.ingrediants);

}