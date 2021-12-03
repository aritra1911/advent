use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

fn main() {

    let args: Vec<String> = env::args().collect();

    let report: Vec<String> = if args.len() > 1 && args[1] != "-" {
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

    let (gamma_rate, epsilon_rate) = calculate_rates(&report);
    println!("Answer to Part One : {}", gamma_rate * epsilon_rate);

    let oxygen_generator_rating = calculate_oxygen_rating(&report);
    let co2_scrubber_rating = calculate_co2_scrubber_rating(&report);
    println!("Answer to Part Two : {}",
             oxygen_generator_rating * co2_scrubber_rating);
}

fn calculate_rates(report: &Vec<String>) -> (u32, u32) {

    let mut ones: Vec<usize> = Vec::new();
    let nums: usize = report.len();

    for bin_number in report {
        for (idx, bit) in bin_number.chars().enumerate() {

            let is_one: usize = match bit {
                '0' => 0,
                '1' => 1,
                _ => unreachable!(),
            };

            if idx == ones.len() {
                ones.push(is_one);
            } else {
                ones[idx] += is_one;
            }
        }
    }

    /* A simple check to see if we got equal number of ones and zeros */
    if nums & 1 == 0 {
        for n in ones.clone() {
            assert_ne!(n, nums / 2);
        }
    }
    let n_bits = ones.len();

    let mut gamma_rate = 0;
    for n in ones {
        gamma_rate <<= 1;

        if n > nums / 2 {
            gamma_rate |= 1;
        }
    }

    let epsilon_rate = !gamma_rate & ((1 << n_bits) - 1);

    (gamma_rate, epsilon_rate)
}

fn calculate_oxygen_rating(report: &Vec<String>) -> u32 {

    let mut numbers: Vec<String> = report.clone();
    let mut idx = 0;
    let mut nums = numbers.len();

    loop {
        let mut ones = 0usize;

        for bin_number in numbers.clone() {
            if bin_number.as_bytes()[idx] == '1' as u8 { ones += 1 }
        }

        let maj = if {
            let zeros = nums - ones;
            ones >= zeros
        } { '1' as u8 } else { '0' as u8 };

        numbers = numbers.into_iter()
                         .filter(|num| num.as_bytes()[idx] == maj)
                         .collect();

        nums = numbers.len();

        if nums == 1 {
            break;
        }

        idx += 1;
    }

    let num = numbers.pop().unwrap();
    u32::from_str_radix(num.as_str(), 2).unwrap()
}

fn calculate_co2_scrubber_rating(report: &Vec<String>) -> u32 {

    let mut numbers: Vec<String> = report.clone();
    let mut idx = 0;
    let mut nums = numbers.len();

    loop {
        let mut ones = 0usize;

        for bin_number in numbers.clone() {
            if bin_number.as_bytes()[idx] == '1' as u8 { ones += 1 }
        }

        let min = if {
            let zeros = nums - ones;
            ones < zeros
        } { '1' as u8 } else { '0' as u8 };

        numbers = numbers.into_iter()
                         .filter(|num| num.as_bytes()[idx] == min)
                         .collect();

        nums = numbers.len();

        if nums == 1 {
            break;
        }

        idx += 1;
    }

    let num = numbers.pop().unwrap();
    u32::from_str_radix(num.as_str(), 2).unwrap()
}
