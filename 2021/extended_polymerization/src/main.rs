use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use itertools::Itertools;
use std::collections::HashMap;
use std::str;

fn pair_insert(polymer: &mut Vec<u8>, rules: &HashMap<&str, u8>) {

    let mut new_elements = Vec::new();

    for i in 0..(polymer.len() - 1) {
        let pair = [polymer[i], polymer[i + 1]];
        let pair = str::from_utf8(&pair).unwrap();
        new_elements.push(rules.get(pair).unwrap());
    }

    let mut new_polymer = Vec::new();
    for (&a, &b) in polymer.iter().zip(new_elements) {
        new_polymer.push(a);
        new_polymer.push(b);
    }
    new_polymer.push(*polymer.last().unwrap());

    *polymer = new_polymer.clone();
}

#[allow(unused_variables)]
fn part1(template: &String, rules: &HashMap<&str, u8>) -> usize {

    let mut polymer = template
        .chars()
        .map(|c| c as u8)
        .collect();

    let steps = 10;
    for step in 1..=steps {
        pair_insert(&mut polymer, &rules);
    }

    let mut distinct_elements = polymer.clone();
    distinct_elements.sort_unstable();
    distinct_elements.dedup();

    let (mut min, mut max) = (usize::MAX, 0);
    for element in distinct_elements {
        let count = polymer.iter().filter(|&&e| e == element).count();
        if count > max { max = count; }
        if count < min { min = count; }
    }

    max - min
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let lines: Vec<String> = if args.len() > 1 && args[1] != "-" {
        let file = File::open(&args[1]).unwrap();
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(|line| line.unwrap().trim().to_string())
            .collect()
    } else {
        stdin()
            .lock()
            .lines()
            .map(|line| line.unwrap().trim().to_string())
            .collect()
    };

    /* Parse all of that into a HashMap */
    let template = lines.first().unwrap();
    let mut rules = HashMap::new();
    for i in 2..lines.len() {
        let (key, value) = lines[i].split(" -> ").collect_tuple().unwrap();
        let value = value.chars().next().unwrap() as u8;
        rules.insert(key, value);
    }

    let difference = part1(&template, &rules);
    println!("Answer to Part One : {}", difference);
}
