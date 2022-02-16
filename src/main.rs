

use rayon::iter::{IntoParallelIterator, ParallelIterator};

mod optimizer;
mod util;
mod preferences;

fn main() {
    optimizer::find_by_random();
    loop {
        println!("new iteration!");
        optimizer::find_by_walking();
    }
    println!("End");
}
