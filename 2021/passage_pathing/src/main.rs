use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::str::FromStr;
use std::num::ParseIntError;
use itertools::Itertools;

struct Path(String, String);

struct Cave {
    name: String,
    connected_caves: Vec<usize>,
}

impl FromStr for Path {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.trim()
                            .split("-")
                            .collect_tuple()
                            .unwrap();

        let start = String::from(start);
        let end = String::from(end);

        Ok(Path(start, end))
    }
}

fn cave_is_small(caves: &Vec<Cave>, idx: usize) -> bool {
    caves[idx].name
        .chars()
        .next().unwrap()
        .is_lowercase()
}

fn bfs(  caves: &Vec<Cave>,   idx: usize,
       visited: Vec<u8>,    times: u8) -> u64 {

    if caves[idx].name == "end" {
        return 1;
    }

    /* Append the fact that this cave has been visited,
     * but push if and only if the cave is small.
     * Also if we visit a cave `times` times, then
     * `next_time` shall be `times` - 1. */
    let mut visited = visited.clone();
    let mut next_times = times;
    if cave_is_small(caves, idx) {
        visited[idx] += 1;
        if times > 1 && visited[idx] == times {
            next_times -= 1;
        }
    }

    /* Get queue of connected caves to visit. */
    let mut queue = caves[idx].connected_caves.clone();

    let mut distinct_paths = 0;
    loop {
        match queue.pop() {
            Some (idx) => {
                /* Check if already visited in case of small cave */
                if { let is_small = cave_is_small(caves, idx);
                     !is_small || (is_small && visited[idx] < next_times) } {

                    distinct_paths +=
                        bfs(caves, idx, visited.clone(), next_times);
                }
            },

            None => { break; },
        }
    }

    distinct_paths
}

fn get_cave_index(caves: &mut Vec<Cave>, name: &String) -> usize {

    let n_caves = caves.len();

    for i in 0..n_caves {
        if caves[i].name == *name {
            return i;
        }
    }

    caves.push(Cave {
        name: name.clone(),
        connected_caves: Vec::new(),
    });

    n_caves
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let paths: Vec<Path> = if args.len() > 1 && args[1] != "-" {
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

    /* Prepare caves graph */
    let mut caves = Vec::new();
    for path in paths {
        let Path(start, end) = path;
        let i = get_cave_index(&mut caves, &start);
        let j = get_cave_index(&mut caves, &end);

        /* Prevent paths back to "start" cave, and
         * prevent paths from "end" cave. */
        if caves[j].name != "start" && caves[i].name != "end" {
            caves[i].connected_caves.push(j);
        }
        if caves[i].name != "start" && caves[j].name != "end" {
            caves[j].connected_caves.push(i);
        }
    }

    /* How caves graph looks like:
     *
     * index      0 1 2 3 4   5
     *  cave  start A b c d end
     * conns      1 3 1 1 2
     *            2 2 4
     *              5 5
     */

    let n_caves = caves.len();

    /* Determine start index */
    let mut start_idx = 0;
    for i in 0..n_caves {
        if caves[i].name == "start" {
            start_idx = i;
        }
    }

    /* Apply BFS and let it do its magic */

    let distinct_paths = bfs(&caves, start_idx, vec![0; n_caves], 1);
    println!("Answer to Part One : {}", distinct_paths);

    let distinct_paths = bfs(&caves, start_idx, vec![0; n_caves], 2);
    println!("Answer to Part Two : {}", distinct_paths);
}
