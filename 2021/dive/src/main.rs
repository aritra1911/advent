use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::str::FromStr;
use std::num::ParseIntError;
use itertools::Itertools;

#[derive(Debug)]
enum Command {
    FORWARD(u32),
    DOWN(u32),
    UP(u32),
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, units) = s.trim().split(' ').collect_tuple().unwrap();
        let units: u32 = units.parse()?;

        match dir {
            "forward" => Ok(Command::FORWARD(units)),
            "down" => Ok(Command::DOWN(units)),
            "up" => Ok(Command::UP(units)),
            &_ => unreachable!(),
        }
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let commands_vector: Vec<Command> = if args.len() > 1 && args[1] != "-" {
        let file = File::open(&args[1]).unwrap();
        let reader = BufReader::new(file);

        reader.lines()
              .map(|line| line.unwrap().trim().parse().unwrap())
              .collect()
    } else {
        stdin().lock()
               .lines()
               .map(|line| line.unwrap().trim().parse().unwrap())
               .collect()
    };

    let (h_pos, depth) = calculate_position(&commands_vector);
    println!("Answer to Part One : {}", h_pos * depth);

    let (h_pos, depth, _) = calculate_pos_and_aim(&commands_vector);
    println!("Answer to Part Two : {}", h_pos * depth);
}

fn calculate_position(commands_vector: &Vec<Command>) -> (i32, i32) {

    let mut h_pos: i32 = 0;
    let mut depth: i32 = 0;

    for cmd in commands_vector {
        match cmd {
            Command::FORWARD(units) => {
                let units = *units;
                h_pos += units as i32;
            },
            Command::DOWN(units) => {
                let units = *units;
                depth += units as i32;
            },
            Command::UP(units) => {
                let units = *units;
                depth -= units as i32;
            },
        }
    }

    (h_pos, depth)
}

fn calculate_pos_and_aim(commands_vector: &Vec<Command>) -> (i32, i32, i32) {

    let mut h_pos: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for cmd in commands_vector {
        match cmd {
            Command::FORWARD(units) => {
                let units = *units as i32;
                h_pos += units;
                depth += aim * units;
            },
            Command::DOWN(units) => {
                let units = *units as i32;
                aim += units;
            },
            Command::UP(units) => {
                let units = *units as i32;
                aim -= units;
            }
        }
    }

    (h_pos, depth, aim)
}
