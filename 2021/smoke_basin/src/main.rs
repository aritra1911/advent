use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

fn main() {

    let args: Vec<String> = env::args().collect();

    let heightmap: Vec<Vec<u8>> = if args.len() > 1 && args[1] != "-" {
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

    let low_points = get_low_points(&heightmap);
    let sum: u64 = low_points.iter()
                             .map(|(i, j)| heightmap[*i][*j] as u64 + 1)
                             .sum();
    println!("Answer to Part One : {}", sum);

    let mut basin_sizes = Vec::new();
    let map_height = heightmap.len();
    let map_width = heightmap[0].len();
    let mut visited = vec![vec![false; map_width]; map_height];
    for point in low_points {
        let size = get_basin_size(&heightmap, point, &mut visited);
        basin_sizes.push(size);
    }

    basin_sizes.sort_unstable();
    let mut product = basin_sizes.pop().unwrap();
    product *= basin_sizes.pop().unwrap();
    product *= basin_sizes.pop().unwrap();
    println!("Answer to Part Two : {}", product);
}

fn get_low_points(heightmap: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {

    let mut low_points = Vec::new();
    let map_height = heightmap.len();
    let map_width = heightmap[0].len();

    for i in 0..map_height {
        for j in 0..map_width {

            let height = heightmap[i][j];

            let lower_than_up = if i > 0 {
                heightmap[i - 1][j] > height
            } else { true };

            let lower_than_down = if i < map_height - 1 {
                heightmap[i + 1][j] > height
            } else { true };

            let lower_than_left = if j > 0 {
                heightmap[i][j - 1] > height
            } else { true };

            let lower_than_right = if j < map_width - 1 {
                heightmap[i][j + 1] > height
            } else { true };

            if lower_than_up && lower_than_down &&
               lower_than_left && lower_than_right {
                low_points.push((i, j));
            }
        }
    }

    low_points
}

fn get_basin_size(heightmap: &Vec<Vec<u8>>, point: (usize, usize),
                  visited: &mut Vec<Vec<bool>>) -> u64 {

    let (i, j) = point;
    let height = heightmap[i][j];
    let map_height = heightmap.len();
    let map_width = heightmap[0].len();

    if height == 9 || visited[i][j] {
        return 0;
    }

    visited[i][j] = true;
    let mut size = 1;

    /* Up */
    if i > 0 {
        size += get_basin_size(heightmap, (i - 1, j), visited);
    }

    /* Down */
    if i < map_height - 1 {
        size += get_basin_size(heightmap, (i + 1, j), visited);
    }

    /* Left */
    if j > 0 {
        size += get_basin_size(heightmap, (i, j - 1), visited);
    }

    /* Right */
    if j < map_width - 1 {
        size += get_basin_size(heightmap, (i, j + 1), visited);
    }

    size
}
