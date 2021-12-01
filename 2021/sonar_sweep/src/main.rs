use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {

    let args: Vec<String> = env::args().collect();

    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);

    let depth_vector: Vec<u32> = reader.lines()
                                       .map(|depth| depth.unwrap()
                                                         .trim()
                                                         .parse()
                                                         .unwrap())
                                       .collect();

    println!("{}", get_increments(&depth_vector));
}

fn get_increments(depth_vector: &Vec<u32>) -> u32 {

    let mut increments = 0;

    for (prev_depth, depth) in depth_vector.iter().tuple_windows() {
        if prev_depth < depth {
            increments += 1;
        }
    }

    increments
}
