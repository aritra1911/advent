use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::cmp::min;

fn get_neighbours(   i: usize,    j: usize,
                  rows: usize, cols: usize) -> Vec<(usize, usize)> {

    let mut neighbours = Vec::new();

    if i > 0        { neighbours.push((i - 1, j)); }
    if i < rows - 1 { neighbours.push((i + 1, j)); }
    if j > 0        { neighbours.push((i, j - 1)); }
    if j < cols - 1 { neighbours.push((i, j + 1)); }

    neighbours
}

fn dijkstra(risk_levels: &Vec<Vec<u8>>, source: (usize, usize)) -> Vec<Vec<u64>> {

    let rows = risk_levels.len();
    let cols = risk_levels[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut dist = vec![vec![u64::MAX; cols]; rows];
    let mut queue = Vec::new();

    let (src_i, src_j) = source;
    dist[src_i][src_j] = 0;

    /* Add all vertices to queue */
    for i in 0..rows {
        for j in 0..cols {
            if i == src_i && j == src_j { continue; }
            queue.push((i, j));
        }
    }
    queue.push((src_i, src_j));

    while let Some(vertex) = queue.pop() {
        let (i, j) = vertex;

        for neighbour in get_neighbours(i, j, rows, cols) {
            let (n_i, n_j) = neighbour;

            if !visited[n_i][n_j] {
                let new_dist =  dist[i][j] + risk_levels[n_i][n_j] as u64;
                dist[n_i][n_j] = min(new_dist, dist[n_i][n_j]);
            }
        }

        visited[i][j] = true;
        queue.sort_unstable_by(|(i, j), (k, l)| dist[*k][*l].cmp(&dist[*i][*j]));
    }

    dist
}

fn part1(risk_levels: &Vec<Vec<u8>>) -> u64 {

    let distances = dijkstra(risk_levels, (0, 0));
    let rows = risk_levels.len();
    let cols = risk_levels[0].len();

    distances[rows - 1][cols - 1]
}

fn part2(risk_levels: &Vec<Vec<u8>>) -> u64 {

    let rows = risk_levels.len();
    let cols = risk_levels[0].len();
    let mut full_risk_levels = vec![vec![0; cols * 5]; rows * 5];

    for i in 0..rows {
        for j in 0..cols {
            let mut risk_level_rows = risk_levels[i][j];
            for k in 0..5 {
                let mut risk_level_cols = risk_level_rows;
                for l in 0..5 {
                    full_risk_levels[k*rows + i][l*cols + j] = risk_level_cols;
                    risk_level_cols = if risk_level_cols == 9 { 1 }
                                      else { risk_level_cols + 1 };
                }
                risk_level_rows = if risk_level_rows == 9 { 1 }
                                  else { risk_level_rows + 1 };
            }
        }
    }

    let distances = dijkstra(&full_risk_levels, (0, 0));
    distances[5*rows - 1][5*cols - 1]
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let input: Vec<Vec<u8>> = if args.len() > 1 && args[1] != "-" {
        let file = File::open(&args[1]).unwrap();
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(|line| line.unwrap()
                 .trim()
                 .to_string()
                 .chars()
                 .map(|c| c.to_digit(10).unwrap() as u8)
                 .collect())
            .collect()
    } else {
        stdin()
            .lock()
            .lines()
            .map(|line| line.unwrap()
                 .trim()
                 .to_string()
                 .chars()
                 .map(|c| c.to_digit(10).unwrap() as u8)
                 .collect())
            .collect()
    };

    let shortest_distance = part1(&input);
    println!("Answer to Part One : {}", shortest_distance);

    let shortest_distance = part2(&input);
    println!("Answer to Part Two : {}", shortest_distance);
}
