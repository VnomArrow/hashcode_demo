#[derive(Debug, Default)]
pub struct Person {
    pub likes : Vec<usize>,
    pub dislikes : Vec<usize>,
}

#[inline]
pub fn is_valid(person : &Person, pizza: &Vec<usize>) -> bool {
    for topping in &person.likes {
        if !pizza.contains(topping)  {
            return false;
        }
    }

    for topping in &person.dislikes {
        if pizza.contains(topping)  {
            return false;
        }
    }

    true
}

// Get the value of the pizza, how many people that can eat it
pub fn solve(list : &Vec<Person>, pizza: &Vec<usize>) -> usize {
    let mut valid_ppl = 0usize;
    for person in list {
        if is_valid(person, pizza) {
            valid_ppl += 1;
        }
    }
    valid_ppl
}

pub mod alternate_solve {
    use std::io::Write;
    use crate::victor_code::*;

    use rand::prelude::ThreadRng;

    use crate::vincent_code::*;
    #[derive(Debug, Default)]
    pub struct Solver {
        ingrediants: Vec<Ingrediant>,
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
    impl Solver {
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
    }
}