use crate::optimizer::*;
use crate::preferences::*;
use crate::util::*;

pub fn walk_steep(preference_info: &PizzaPreferenceInfo, pizza_in: &Pizza) -> Pizza {
    // Take a random pizza, then try and change each ingrediant, the one that gives the highest score is changed, then the process repeats

    let solver = CustomerCounter::new(&preference_info.persons, preference_info.ingrediants.len());
    let mut return_pizza = pizza_in.clone();

    loop {
        // Set the best pizza to the previous pizza
        let mut best_pizza = return_pizza.clone();
        let mut best_score = solver.pizza_person_count(&best_pizza, preference_info.persons.len());

        let mut changed = false;
        // Try changing the ingrediants one by one and pick the one that gives the best pizza
        for i in 0..preference_info.ingrediants.len() {
            let mut pizza_copy = return_pizza.clone();
            pizza_copy.ingrediants[i] = !pizza_copy.ingrediants[i];
            let score = solver.pizza_person_count(&pizza_copy, preference_info.persons.len());
            if score > best_score {
                best_pizza = pizza_copy;
                best_score = score;
                changed = true;
                //println!("best {}", best_score)
            }
        }

        if changed {
            return_pizza = best_pizza;
        }
        else {
            break;
        }
    }

    return return_pizza;
}

pub fn walk_first(preference_info: &PizzaPreferenceInfo, pizza_in: &Pizza) -> Pizza {
    // Take a random pizza, then try and change each ingrediant, the one that gives the highest score is changed, then the process repeats

    let solver = CustomerCounter::new(&preference_info.persons, preference_info.ingrediants.len());
    let mut return_pizza = pizza_in.clone();

    loop {
        // Set the best pizza to the previous pizza
        let mut best_pizza = return_pizza.clone();
        let mut best_score = solver.pizza_person_count(&best_pizza, preference_info.persons.len());

        let mut changed = false;
        // Try changing the ingrediants one by one and pick the one that gives the best pizza
        for i in 0..preference_info.ingrediants.len() {
            let mut pizza_copy = best_pizza.clone();
            pizza_copy.ingrediants[i] = !pizza_copy.ingrediants[i];
            let score = solver.pizza_person_count(&pizza_copy, preference_info.persons.len());
            if score > best_score {
                best_pizza = pizza_copy;
                best_score = score;
                changed = true;
                //println!("best {}", best_score)
            }
        }

        if changed {
            return_pizza = best_pizza;
        }
        else {
            break;
        }
    }

    return return_pizza;
}