use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use itertools::Itertools;

fn main() {

    let args: Vec<String> = env::args().collect();

    let depth_vector: Vec<u32> = if args.len() > 1 && args[1] != "-" {
        let file = File::open(&args[1]).unwrap();
        let reader = BufReader::new(file);

        reader.lines()
              .map(|depth| depth.unwrap()
                                .trim()
                                .parse()
                                .unwrap())
              .collect()
    } else {
        stdin().lock()
               .lines()
               .map(|depth| depth.unwrap()
                                 .trim()
                                 .parse()
                                 .unwrap())
               .collect()
    };

    println!("Answer to Part One : {}", get_increments(&depth_vector));

    let window_sums_vector = get_window_sums(&depth_vector);
    println!("Answer to Part Two : {}", get_increments(&window_sums_vector));
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

fn get_window_sums(depth_vector: &Vec<u32>) -> Vec<u32> {

    let mut window_sums = Vec::new();

    for (first, second, third) in depth_vector.iter().tuple_windows() {
        window_sums.push(first + second + third);
    }

    window_sums
}
