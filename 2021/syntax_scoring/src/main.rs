use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

const BRACKET_TYPES: usize = 4;

#[derive(Debug)]
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

fn is_corrupted(line: &String) -> (Option<Bracket>, Vec<char>) {

    let mut count = [0u64; BRACKET_TYPES];
    let mut expected_closing = '\0';
    let mut closings = Vec::new();

    for c in line.chars() {
        match c {

            /* Opening / Insertions */
            '(' => {
                count[Bracket::PARENTHESIS as usize] += 1;
                if expected_closing != '\0' {
                    closings.push(expected_closing);
                }
                expected_closing = ')';
            },
            '[' => {
                count[Bracket::SQUARE as usize] += 1;
                if expected_closing != '\0' {
                    closings.push(expected_closing);
                }
                expected_closing = ']';
            },
            '{' => {
                count[Bracket::CURLY as usize] += 1;
                if expected_closing != '\0' {
                    closings.push(expected_closing);
                }
                expected_closing = '}';
            },
            '<' => {
                count[Bracket::ANGULAR as usize] += 1;
                if expected_closing != '\0' {
                    closings.push(expected_closing);
                }
                expected_closing = '>';
            },

            /* Closing / Deletions */
            ')' => {
                if count[Bracket::PARENTHESIS as usize] > 0 &&
                   expected_closing == c {
                    count[Bracket::PARENTHESIS as usize] -= 1;
                    expected_closing = match closings.pop() {
                        Some(a) => a,
                        None => '\0',
                    }
                } else {
                    if expected_closing != '\0' {
                        closings.push(expected_closing);
                    }
                    return (Some(Bracket::PARENTHESIS), closings);
                }
            },
            ']' => {
                if count[Bracket::SQUARE as usize] > 0 &&
                   expected_closing == c {
                    count[Bracket::SQUARE as usize] -= 1;
                    expected_closing = match closings.pop() {
                        Some(a) => a,
                        None => '\0',
                    }
                } else {
                    if expected_closing != '\0' {
                        closings.push(expected_closing);
                    }
                    return (Some(Bracket::SQUARE), closings);
                }
            },
            '}' => {
                if count[Bracket::CURLY as usize] > 0 &&
                   expected_closing == c {
                    count[Bracket::CURLY as usize] -= 1;
                    expected_closing = match closings.pop() {
                        Some(a) => a,
                        None => '\0',
                    }
                } else {
                    if expected_closing != '\0' {
                        closings.push(expected_closing);
                    }
                    return (Some(Bracket::CURLY), closings);
                }
            },
            '>' => {
                if count[Bracket::ANGULAR as usize] > 0 &&
                   expected_closing == c {
                    count[Bracket::ANGULAR as usize] -= 1;
                    expected_closing = match closings.pop() {
                        Some(a) => a,
                        None => '\0',
                    }
                } else {
                    if expected_closing != '\0' {
                        closings.push(expected_closing);
                    }
                    return (Some(Bracket::ANGULAR), closings);
                }
            },

            _ => unreachable!(),
        }
    }

    if expected_closing != '\0' {
        closings.push(expected_closing);
    }

    (None, closings)
}

fn part1(lines: &Vec<String>) -> u64 {

    let mut errors = [0u64; BRACKET_TYPES];

    for line in lines {
        if let (Some(bracket), _) = is_corrupted(line) {
            errors[bracket as usize] += 1;
        }
    }

    errors
        .iter()
        .enumerate()
        .map(|(b, count)| count * SCORE[b as usize])
        .sum()
}

fn part2(lines: &Vec<String>) -> u64 {

    let mut scores = Vec::new();

    for line in lines {
        match is_corrupted(line) {
            (None, mut closings) => {
                if closings.len() > 0 {
                    /* Incomplete line */
                    let mut score = 0;

                    while let Some(c) = closings.pop() {
                        score *= 5;
                        score += match c {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => unreachable!(),
                        };
                    }

                    scores.push(score);
                }
            },

            _ => { },
        }
    }

    /* > There will always be an odd number of scores to consider.
     * I take no chances! */
    let len = scores.len();
    assert_eq!(len & 1, 1);

    scores.sort();
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
