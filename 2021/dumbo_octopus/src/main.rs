use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

const GRID_DIM: usize = 10;
const OCTOPUSES: usize = GRID_DIM * GRID_DIM;

fn step_octopuses(energy_map: &mut [u8]) {
    for i in 0..OCTOPUSES {
        energy_map[i] += 1;
    }
}

fn get_adjacents(octopus: (usize, usize)) -> Vec<(usize, usize)> {

    let mut adjacents = Vec::new();
    let (i, j) = octopus;

    if i > 0 {
        if j > 0 {
            adjacents.push((i - 1, j - 1));
        }
        adjacents.push((i - 1, j));
        if j < GRID_DIM - 1 {
            adjacents.push((i - 1, j + 1));
        }
    }

    if j > 0 {
        adjacents.push((i, j - 1));
    }

    if j < GRID_DIM - 1 {
        adjacents.push((i, j + 1));
    }

    if i < GRID_DIM - 1 {
        if j > 0 {
            adjacents.push((i + 1, j - 1));
        }
        adjacents.push((i + 1, j));
        if j < GRID_DIM - 1 {
            adjacents.push((i + 1, j + 1));
        }
    }

    adjacents
}

fn flash(energy_map: &mut [u8], octopus: (usize, usize)) -> u64 {

    let (i, j) = octopus;

    if energy_map[i * GRID_DIM + j] <= 9 {
        return 0;
    }

    let mut flashes = 1;
    energy_map[i * GRID_DIM + j] = 0;

    for octopus in get_adjacents(octopus) {
        let (i, j) = octopus;
        if energy_map[i * GRID_DIM + j] > 0 {
            energy_map[i * GRID_DIM + j] += 1;
        }
        flashes += flash(energy_map, octopus);
    }

    flashes
}

#[allow(dead_code)]
fn dump_energy_map(energy_map: &[u8], step: u64) {

    println!("\nAfter step {:3}:", step);
    for i in 0..GRID_DIM {
        for j in 0..GRID_DIM {
            print!("{}", energy_map[i * GRID_DIM + j]);
        }
        println!();
    }
}

#[allow(unused_variables)]
fn part1(energy_map: &mut [u8], steps:u64) -> u64 {

    let mut total_flashes = 0;

    for step in 0..steps {
        step_octopuses(energy_map);
        for i in 0..GRID_DIM {
            for j in 0..GRID_DIM {
                total_flashes += flash(energy_map, (i, j));
            }
        }
        //dump_energy_map(energy_map, step + 1);
    }

    total_flashes
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let input: Vec<Vec<u8>> = if args.len() > 1 && args[1] != "-" {
        let file = File::open(&args[1]).unwrap();
        let reader = BufReader::new(file);

        reader.lines()
            .map(|line| line.unwrap()
                 .trim()
                 .to_string()
                 .chars()
                 .map(|c| c.to_digit(10).unwrap() as u8)
                 .collect())
            .collect()
    } else {
        stdin().lock()
            .lines()
            .map(|line| line.unwrap()
                 .trim()
                 .to_string()
                 .chars()
                 .map(|c| c.to_digit(10).unwrap() as u8)
                 .collect())
            .collect()
    };

    /* Copy vector to a linear array which is way easier to work with */
    let mut energy_map = [0u8; GRID_DIM * GRID_DIM];
    for i in 0..GRID_DIM {
        for j in 0..GRID_DIM {
            energy_map[i * GRID_DIM + j] = input[i][j];
        }
    }

    let flashes = part1(&mut energy_map, 100);
    println!("Answer to Part One : {}", flashes);
    /*
    println!("Answer to Part Two : {}", step);
    */
}
