use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::str::FromStr;
use std::num::ParseIntError;
use itertools::Itertools;

const DIGITS: usize = 4;

#[derive(Clone, Debug)]
struct Entry {
    patterns: Vec<String>,
    output: Vec<String>,
}

impl FromStr for Entry {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let (patterns, output) = s.trim()
                                  .split(" | ")
                                  .collect_tuple().unwrap();

        let patterns: Vec<String> = patterns.trim()
                                            .split_whitespace()
                                            .map(|x| x.to_string())
                                            .collect();

        let output: Vec<String> = output.trim()
                                        .split_whitespace()
                                        .map(|x| x.to_string())
                                        .collect();

        Ok(Entry { patterns: patterns, output: output, })
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let entries: Vec<Entry> = if args.len() > 1 && args[1] != "-" {
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

    let unique_digits = part1(&entries);
    //println!("Answer to Part One : {}", unique_digits);

    let sum = part2(&entries);
    //println!("Answer to Part Two : {}", sum);
}

fn part1(entries: &Vec<Entry>) -> u64 {

    let mut unique_digits = 0;

    for entry in entries {
        unique_digits += get_unique_digits(&entry.output);
    }

    unique_digits
}

fn get_unique_digits(output: &Vec<String>) -> u64 {

    let mut unique_digits = 0;

    let is_unique = |digit: String| match digit.len() {
        2 | 4 | 3 | 7 => true,
        _ => false,
    };

    for digit in output {
        if is_unique(digit.to_string()) {
            unique_digits += 1;
        }
    }

    unique_digits
}

fn part2(entries: &Vec<Entry>) -> u64 {

    for entry in entries {
        let map = remap(&entry.patterns);
    }

    0
}

fn remap(patterns: &Vec<String>) -> [usize; 10] {

    let mut map = [0; 10];  /* Table of indexes */
    let mut segments = ['a'; 7];
    let mut possible_rights = ['a', 'a'];
    let mut possible_bottoms = ['a', 'a'];
    let mut possible_mids = ['a', 'a'];
    let mut collapsed_rights = false;
    let mut collapsed_bottoms = false;
    let mut collapsed_mids = false;
    let mut i = 0;

    for i in 0..10 {
        match patterns[i].len() {
            2 => { map[1] = i; },
            3 => { map[7] = i; },
            4 => { map[4] = i; },
            7 => { map[8] = i; },
            _ => { },
        }
    }

    /* Missing segment in digit 1 appears on digit 7 */
    for c in patterns[map[7]].chars() {
        if patterns[map[1]].contains(c) {
            possible_rights[i] = c;
            i += 1;
        } else {
            segments[0] = c;  /* Top Segment */
        }
    }
    i = 0;

    println!("Top Segment : {}", segments[0]);
    let mut possible_rights = (possible_rights[0], possible_rights[1]);
    println!("Possible Rights : {:?}", possible_rights);

    /* Comparing segments for 4 and 7 gives possibilities
     * for top right and bottom right segments */
    for c in patterns[map[4]].chars() {
        if !patterns[map[7]].contains(c) {
            possible_mids[i] = c;
            i += 1;
        }
    }
    i = 0;

    let mut possible_mids = (possible_mids[0], possible_mids[1]);
    println!("Possible Mids : {:?}", possible_mids);

    /* Possibilities for bottom-left and bottom segments can be inferred
     * by segments missing from digit 8 and other possibility lists */
    for c in patterns[map[8]].chars() {
        if c != segments[0] &&
           possible_mids.0 != c && possible_mids.1 != c &&
           possible_rights.0 != c && possible_rights.1 != c {

            possible_bottoms[i] = c;
            i += 1;
        }
    }
    i = 0;

    let mut possible_bottoms = (possible_bottoms[0], possible_bottoms[1]);
    println!("Possible Bottoms : {:?}", possible_bottoms);

    /* a b c d e f g *
     * 0 1 2 3 4 5 6 */

    for (i, pattern) in patterns.iter().enumerate() {
        let len = pattern.len();

        if len != 2 && len != 3 && len != 4 && len != 7 {
            /* Possibilities : 0, 2, 3, 5, 6, 9 */
            if len == 5 {
                /* Possibilities : 2, 3, 5 */
                let (p, q) = possible_rights;
                let (r, s) = possible_mids;

                if pattern.contains(p) && pattern.contains(q) {
                    /* It's a 3 */
                    map[3] = i;

                    if !collapsed_mids {
                        /* collapse middle and top left segments */
                        possible_mids = if pattern.contains(s) { (r, s) }
                                        else { (s, r) };
                        collapsed_mids = true;
                        segments[1] = possible_mids.0;
                        segments[3] = possible_mids.1;
                    }

                    if !collapsed_bottoms {
                        /* collapse bottom-left and bottom segments */
                        let (t, u) = possible_bottoms;
                        possible_bottoms = if pattern.contains(u) { (t, u) }
                                           else { (u, t) };
                        collapsed_bottoms = true;
                        segments[4] = possible_bottoms.0;
                        segments[6] = possible_bottoms.1;
                    }

                } else if pattern.contains(r) && pattern.contains(s) {
                    /* It's a 5 */
                    map[5] = i;

                    if !collapsed_rights {
                        possible_rights = if pattern.contains(q) { (p, q) }
                                          else { (q, p) };
                        collapsed_rights = true;
                        segments[2] = possible_rights.0;
                        segments[5] = possible_rights.1;
                    }

                    if !collapsed_bottoms {
                        /* collapse bottom-left and bottom segments */
                        let (t, u) = possible_bottoms;
                        possible_bottoms = if pattern.contains(u) { (t, u) }
                                           else { (u, t) };
                        collapsed_bottoms = true;
                        segments[4] = possible_bottoms.0;
                        segments[6] = possible_bottoms.1;
                    }

                } else {
                    /* It's a 2 */
                    map[2] = i;

                    if !collapsed_rights {
                        possible_rights = if pattern.contains(q) { (p, q) }
                                          else { (q, p) };
                        collapsed_rights = true;
                        segments[2] = possible_rights.0;
                        segments[5] = possible_rights.1;
                    }

                    if !collapsed_mids {
                        /* collapse top-left and middle segments */
                        possible_mids = if pattern.contains(s) { (r, s) }
                                        else { (s, r) };
                        collapsed_mids = true;
                        segments[1] = possible_mids.0;
                        segments[3] = possible_mids.1;
                    }

                }
            } else if len == 6 {
                /* Possibilities : 0, 6, 9 */
                let (p, q) = possible_rights;
                let (t, u) = possible_bottoms;

                /* Logical XOR is a myth nowadays */
                if { let has_p = pattern.contains(p);
                     let has_q = pattern.contains(q);
                     (has_p && !has_q) || (has_q && !has_p) } {

                    /* It's a 6 */
                    map[6] = i;

                    if !collapsed_rights {
                        possible_rights = if pattern.contains(q) { (p, q) }
                                          else { (q, p) };
                        collapsed_rights = true;
                        segments[2] = possible_rights.0;
                        segments[5] = possible_rights.1;
                    }

                } else if { let has_t = pattern.contains(t);
                            let has_u = pattern.contains(u);
                            (has_t && !has_u) || (has_u && !has_t) } {

                    /* It's a 9 */
                    map[9] = i;

                    if !collapsed_bottoms {
                        /* collapse bottom-left and botoom segments */
                        possible_bottoms = if pattern.contains(u) { (t, u) }
                                           else { (u, t) };
                        collapsed_bottoms = true;
                        segments[4] = possible_bottoms.0;
                        segments[6] = possible_bottoms.1;
                    }

                } else {
                    /* It's a 0 */
                    map[0] = i;

                    if !collapsed_mids {
                        /* collapse top-left and middle segments */
                        let (r, s) = possible_mids;
                        possible_mids = if pattern.contains(r) { (r, s) }
                                        else { (s, r) };
                        collapsed_mids = true;
                        segments[1] = possible_mids.0;
                        segments[3] = possible_mids.1;
                    }
                }
            }
        }

        if collapsed_rights && collapsed_bottoms && collapsed_mids { break; }
    }


    println!("             Map : {:?}", map);
    println!("        Segments : {:?}", segments);
    println!("Possible  Rights : {:?}", possible_rights);
    println!("Possible    Mids : {:?}", possible_mids);
    println!("Possible Bottoms : {:?}", possible_bottoms);

    println!();

    map
}
