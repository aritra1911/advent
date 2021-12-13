use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use itertools::Itertools;

fn transform_points(points: &mut Vec<(u32, u32)>, fold: (char, u32)) {

    for i in 0..points.len() {
        let (x, y) = points[i];

        if fold.0 == 'y' && y > fold.1 {
            points[i] = (x, 2 * fold.1 - y);
        } else if fold.0 == 'x' && x > fold.1 {
            points[i] = (2 * fold.1 - x, y);
        }
    }

    points.sort_unstable();
    points.dedup();
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

    /* Parse points */
    let mut points: Vec<(u32, u32)> = Vec::new();
    let mut folds_idx = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() {
            folds_idx = i + 1;
            break;
        }
        let (x, y) = line.split(",").collect_tuple().unwrap();
        points.push((x.parse().unwrap(), y.parse().unwrap()));
    }

    /* Parse fold instructions */
    let mut folds: Vec<(char, u32)> = Vec::new();
    for i in folds_idx..lines.len() {
        let mut stripped_line = lines[i].chars().skip(11);
        let axis = stripped_line.next().unwrap();
        stripped_line.next().unwrap();  /* skip '=' sign */
        let pos = stripped_line
            .collect::<String>()
            .parse().unwrap();
        folds.push((axis, pos));
    }

    transform_points(&mut points, folds[0]);
    println!("Answer to Part One : {}", points.len());

    for i in 1..folds.len() {
        transform_points(&mut points, folds[i]);
    }
    println!("Answer to Part Two :");
    for point in points {
        /* TODO: Plot some graphics here instead */
        print!("{:?}, ", point);
    }
    println!();
}
