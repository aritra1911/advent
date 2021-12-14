use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use itertools::Itertools;
use std::collections::HashMap;

fn pair_insert(polymer_pairs: &mut HashMap<String, usize>,
                       rules: &HashMap<&str, u8>) {

    let mut new_pairs: HashMap<String, usize> = HashMap::new();

    for (pair, count) in polymer_pairs.iter() {
        let element = rules.get(pair.as_str()).unwrap();
        let a = pair.as_bytes()[0];
        let b = pair.as_bytes()[1];
        let first_pair_arr = vec![a, *element];
        let second_pair_arr = vec![*element, b];
        let first_pair = String::from_utf8(first_pair_arr).unwrap();
        let second_pair = String::from_utf8(second_pair_arr).unwrap();

        match new_pairs.get_mut(&first_pair) {
            Some(v) => { *v += *count; },
            None => { new_pairs.insert(first_pair, *count); },
        }

        match new_pairs.get_mut(&second_pair) {
            Some(v) => { *v += *count; },
            None => { new_pairs.insert(second_pair, *count); },
        }
    }

    *polymer_pairs = new_pairs.clone();
}

#[allow(unused_variables)]
fn solve(template: &String, rules: &HashMap<&str, u8>, steps: usize) -> usize {

    let mut polymer_pairs = HashMap::new();

    for i in 0..(template.len() - 1) {
        let bytes = template.as_bytes();
        let pair_arr = vec![bytes[i], bytes[i + 1]];
        let pair = String::from_utf8(pair_arr).unwrap();

        match polymer_pairs.get_mut(&pair) {
            Some(count) => { *count += 1; },
            None => { polymer_pairs.insert(pair, 1); },
        }
    }

    let last_element = template.as_bytes()[template.len() - 1];

    for step in 1..=steps {
        pair_insert(&mut polymer_pairs, &rules);
    }

    let mut elements: Vec<u8> = polymer_pairs.iter()
        .map(|(pair, _)| pair.as_bytes()[0]).collect();

    elements.push(last_element);
    elements.sort_unstable();
    elements.dedup();

    let (mut max, mut min) = (0, usize::MAX);
    for element in elements {
        let mut total_count = 0;

        for (pair, count) in polymer_pairs.iter() {
            if element == pair.as_bytes()[0] {
                total_count += count;
            }
        }

        /* Don't miss the last element which does not appear
         * within the first characters of the pairs */
        total_count += if element == last_element { 1 } else { 0 };

        if total_count > max { max = total_count; }
        if total_count < min { min = total_count; }
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

    let difference = solve(&template, &rules, 10);
    println!("Answer to Part One : {}", difference);

    let difference = solve(&template, &rules, 40);
    println!("Answer to Part Two : {}", difference);
}
