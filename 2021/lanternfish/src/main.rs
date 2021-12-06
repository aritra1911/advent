use std::env;
use std::fs;
use std::io;

fn main() {

    let args: Vec<String> = env::args().collect();

    let input = if args.len() > 1 && args[1] != "-" {
        fs::read_to_string(&args[1]).unwrap()
    } else {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
                   .expect("Failed to read line");
        input
    };

    let init_state: Vec<u8> = input.split(',')
                                   .map(|x| x.trim().parse().unwrap())
                                   .collect();

    let days = 80;
    let mut freq_state = init_freq_state(&init_state);
    for _ in 0..days {
        spawn(&mut freq_state);
    }
    let n_fishes: u64 = freq_state.iter().sum();
    println!("Answer to Part One : {}", n_fishes);

    let days = 256;
    let mut freq_state = init_freq_state(&init_state);
    for _ in 0..days {
        spawn(&mut freq_state);
    }
    let n_fishes: u64 = freq_state.iter().sum();
    println!("Answer to Part Two : {}", n_fishes);
}

fn init_freq_state(state: &Vec<u8>) -> [u64; 9] {

    let mut freq_state = [0u64; 9];

    /* Calculate frequencies of fishes having same
     * no. of days left to create a new fish */
    for fish in state {
        freq_state[*fish as usize] += 1;
    }

    freq_state
}

fn spawn(freq_state: &mut [u64]) {

    /* Save the # of fishes about to create a new fish */
    let new_fishes = freq_state[0];

    /* Reduce days of every fish by shifting */
    for i in 0..8 {
        freq_state[i] = freq_state[i + 1];
    }

    /* The fishes that created a new fish goes back to 6 days */
    freq_state[6] += new_fishes;

    /* The new fishes get 8 days */
    freq_state[8] = new_fishes;
}
