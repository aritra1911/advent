use std::env;
use std::fs;
use std::io;

fn main() {

    let args: Vec<String> = env::args().collect();

    let input = if args.len() > 1 && args[1] != "-" {
        fs::read_to_string(&args[1]).unwrap()
    } else {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
                   .expect("Failed to read line");
        input
    };

    let positions: Vec<i64> = input.split(',')
                                   .map(|x| x.trim().parse().unwrap())
                                   .collect();

    let target_position = least_fuel_position(&positions, |x| x);
    let total_fuel: i64 = positions.clone()
                                   .into_iter()
                                   .map(|x| (target_position - x).abs())
                                   .sum();

    println!("Answer to Part One : {}", total_fuel);

    let sum_to_n = |x| x * (x + 1) / 2;
    let target_position = least_fuel_position(&positions, sum_to_n);
    let total_fuel: i64 = positions.clone()
                                   .into_iter()
                                   .map(|x| sum_to_n((target_position - x).abs()))
                                   .sum();

    println!("Answer to Part Two : {}", total_fuel);
}

fn least_fuel_position(positions: &Vec<i64>, f: fn(i64) -> i64) -> i64 {

    let mut least_fuel = i64::MAX;
    let mut least_fuel_pos = 0;

    let max_pos = *positions.into_iter().max().unwrap();

    for pos in 0..=max_pos {
        let total_fuel = positions.clone()
                                  .into_iter()
                                  .map(|x| f((pos - x).abs()))
                                  .sum();

        if total_fuel < least_fuel {
            least_fuel = total_fuel;
            least_fuel_pos = pos;
        }
    }

    least_fuel_pos
}
