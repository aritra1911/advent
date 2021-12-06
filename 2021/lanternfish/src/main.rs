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

    let orig_state: Vec<u8> = input.split(',')
                                   .map(|x| x.trim().parse().unwrap())
                                   .collect();

    let mut state = orig_state.clone();
    let days = 80;
    for _ in 0..days {
        spawn(&mut state);
    }
    println!("Answer to Part One : {}", state.len());

    let mut state = orig_state.clone();
    let days = 256;
    for _ in 0..days {
        spawn(&mut state);
    }
    println!("Answer to Part Two : {}", state.len());
}

fn spawn(state: &mut Vec<u8>) {

    let n_fishes = state.len();

    for i in 0..n_fishes {

        if state[i] == 0 {
            state[i] = 6;
            state.push(8);
        } else {
            state[i] -= 1;
        }
    }
}
