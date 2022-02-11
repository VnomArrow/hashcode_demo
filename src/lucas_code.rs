use rand::{prelude::ThreadRng, Rng};

use crate::vincent_code::*;


pub fn runner(persons: &Vec<Person>, max_ingredients: usize) {
    let mut current_pizza: Vec<usize> = vec!();
  
    let mut rng = ThreadRng::default();
  
    for i in 0..rng.gen_range(0..max_ingredients) {
      loop {
        let rng = rng.gen_range(0..max_ingredients);
  
        if current_pizza.contains(&rng) {
          continue;
        }
  
        current_pizza.push(rng);
        break;
      }
    }
  
    let mut previous_best = 0;
  
    loop {
      let mut result = 0;
  
      for i in 0..max_ingredients {
        let mut test_pizza = current_pizza.clone();
        let element_pos = test_pizza.iter().position(|x| *x == i);
        
        match element_pos {
          Some(i) => {
            test_pizza.remove(i);
          },
          None => {
            test_pizza.push(i);
          },
        }
  
        let current_res = solve(&persons, &test_pizza);
  
        if current_res > result { result = current_res };
      }
      
      if result > previous_best {
        previous_best = result;
        continue;
      }
  
      break;
    }
  
    println!("{:?}", previous_best);
  }