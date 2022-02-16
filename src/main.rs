use std::fs::File;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{runner::run, vincent_code::solve};

mod runner;
mod victor_code;
mod vincent_code;

fn main() {
    loop {
        println!("new iteration!");
        victor_code::find_by_walking();
    }
    //victor_code::victor_generate_pizza();
    /*let problem_prefix = "./src/inp/";
    let problems = vec!["a_an_example","b_basic","c_coarse","d_difficult","e_elaborate"];
    //let (persons, count, names) = victor_code::parser(&(problem_prefix.to_string() +"b_basic"));
    //let pizza = vec![2  ,3 ,4 ,5 ,6];
    //let p = solve(&persons, &pizza);
    //println!("{}",p);
    //return;
    problems.into_par_iter().for_each(|prob| {
        if let Ok(_) = std::fs::create_dir(prob) {
            let (persons, count, names) = victor_code::parser(&(problem_prefix.to_string() + &prob));
            run(prob,&persons, count, &names);
        }
    });*/

    println!("End");
}
