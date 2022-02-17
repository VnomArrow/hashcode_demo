

use optimizer::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

mod optimizer;
mod util;
mod preferences;
mod algorithms;

fn main() {
    //print_occurences("d_difficult");
    //optimizer::find_by_random("e_elaborate");
    println!("new iteration!");
    //optimizer::find_by_walking("d_difficult");
    loop {
        find_by_occurances_and_walking("d_difficult");
        //find_by_occurances_and_walking("e_elaborate");
    }
    /*loop {
        println!("new iteration!");
        optimizer::find_by_walking_v2("e_elaborate");
    }
    println!("End");*/
}
