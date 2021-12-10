use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

const BRACKET_TYPES: usize = 4;

#[derive(Copy, Clone, Debug)]
enum Bracket {
    PARENTHESIS,
    SQUARE,
    CURLY,
    ANGULAR,
}

const SCORE: [u64; BRACKET_TYPES] = [
    3,
    57,
    1197,
    25137,
];

fn check(line: &String) -> (Option<Bracket>, Vec<char>) {

    let mut closings = Vec::new();

    for c in line.chars() {
        match c {
            /* Opening / Pushes */
            '(' | '[' | '{' | '<' => {
                let closing_char = match c {
                    '(' => ')',
                    '[' => ']',
                    '{' => '}',
                    '<' => '>',
                    _ => unreachable!(),
                };
                closings.push(closing_char);
            },

            /* Closing / Pops */
            ')' | ']' | '}' | '>' => {
                let bracket = match c {
                    ')' => Bracket::PARENTHESIS,
                    ']' => Bracket::SQUARE,
                    '}' => Bracket::CURLY,
                    '>' => Bracket::ANGULAR,
                    _ => unreachable!(),
                };

                match closings.last() {
                    Some(&a) => if a != c {
                        return (Some(bracket), closings);
                    },
                    None => unreachable!(),
                }
                closings.pop();
            },

            _ => unreachable!(),
        }
    }

    (None, closings)
}

fn part1(lines: &Vec<String>) -> u64 {

    let mut errors = [0u64; BRACKET_TYPES];

    for line in lines {
        if let (Some(bracket), _) = check(line) {
            errors[bracket as usize] += 1;
        }
    }

    errors
        .iter()
        .enumerate()
        .map(|(i, count)| count * SCORE[i])
        .sum()
}

fn part2(lines: &Vec<String>) -> u128 {

    let mut scores = Vec::new();

    for line in lines {
        if let (None, mut closings) = check(line) {
            if closings.len() > 0 {
                /* Incomplete line */

                let mut score = 0;
                while let Some(c) = closings.pop() {
                    score = score * 5 + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    };
                }
                scores.push(score);
            }
        }
    }

    /* > There will always be an odd number of scores to consider.
     * I take no chances! */
    let len = scores.len();
    assert_eq!(len & 1, 1);

    scores.sort_unstable();
    scores[len >> 1]
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let lines: Vec<String> = if args.len() > 1 && args[1] != "-" {
        let file = File::open(&args[1]).unwrap();
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(|line| line.unwrap().trim().parse().unwrap())
            .collect()
    } else {
        stdin()
            .lock()
            .lines()
            .map(|line| line.unwrap().trim().parse().unwrap())
            .collect()
    };

    let score = part1(&lines);
    println!("Answer to Part One : {}", score);

    let score = part2(&lines);
    println!("Answer to Part Two : {}", score);
}
