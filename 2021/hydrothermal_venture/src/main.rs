use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::str::FromStr;
use std::num::ParseIntError;
use itertools::Itertools;

const GRID_DIMENSION: usize = 1024;
const EMPTY_GRID: Grid = [[0; GRID_DIMENSION]; GRID_DIMENSION];

#[derive(Clone, Debug)]
struct Line(u32, u32, u32, u32);

type Grid = [[u32; GRID_DIMENSION]; GRID_DIMENSION];

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x1y1, x2y2) = s.trim()
                            .split(" -> ")
                            .collect_tuple()
                            .unwrap();

        let (x1, y1) = x1y1.trim()
                           .split(',')
                           .map(|x| x.trim()
                                     .parse()
                                     .unwrap())
                           .collect_tuple()
                           .unwrap();

        let (x2, y2) = x2y2.trim()
                           .split(',')
                           .map(|x| x.trim()
                                     .parse()
                                     .unwrap())
                           .collect_tuple()
                           .unwrap();

        Ok(Line(x1, y1, x2, y2))
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let lines: Vec<Line> = if args.len() > 1 && args[1] != "-" {
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

    /* Filter out only the horizontal and vertical lines */
    let lines_hv: Vec<Line> = lines.clone()
                                   .into_iter()
                                   .filter(|Line(x1, y1, x2, y2)|
                                           x1 == x2 || y1 == y2)
                                   .collect();

    let mut grid = EMPTY_GRID;

    for line in lines_hv {
        mark(&mut grid, &line);
    }

    let mut count = 0;
    for i in 0..GRID_DIMENSION {
        for j in 0..GRID_DIMENSION {
            if grid[i][j] > 1 {
                count += 1;
            }
        }
    }

    println!("Answer to Part One : {}", count);

    grid = EMPTY_GRID;

    for line in lines {
        mark(&mut grid, &line);
    }

    count = 0;
    for i in 0..GRID_DIMENSION {
        for j in 0..GRID_DIMENSION {
            if grid[i][j] > 1 {
                count += 1;
            }
        }
    }

    println!("Answer to Part Two : {}", count);
}

fn mark(grid: &mut Grid, line: &Line) {

    let Line(mut x, mut y, x2, y2) = line;

    loop {
        grid[x as usize][y as usize] += 1;

        if x == *x2 && y == *y2 {
            break;
        }

        let x_delta = *x2 as i64 - x as i64;
        let y_delta = *y2 as i64 - y as i64;

        x = if x_delta > 0 { x + 1 }
            else if x_delta < 0 { x - 1 }
            else { x };

        y = if y_delta > 0 { y + 1 }
            else if y_delta < 0 { y - 1 }
            else { y };
    }
}
