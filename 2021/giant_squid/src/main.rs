use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

#[derive(Copy, Clone, Debug)]
struct Cell {
    num: u8,
    marked: bool,
}

impl Cell {
    fn new() -> Self {
        Cell {
            num: 0,
            marked: false,
        }
    }
}

type Board = [[Cell; 5]; 5];

fn main() {

    let args: Vec<String> = env::args().collect();

    let mut lines: Vec<String> = if args.len() > 1 && args[1] != "-" {
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

    let mut boards = Vec::new();

    while lines.len() > 5 {
        boards.push(get_next_board(&mut lines));
    }

    /*
    println!("# of Boards: {}\n", boards.len());
    println!("Board #1:\n{:?}\n", boards[0]);
    println!("Board #2:\n{:?}\n", boards[1]);
    println!("Board #3:\n{:?}\n", boards[2]);
    */

    let draws: Vec<u8> = lines.first()
                              .unwrap()
                              .split(',')
                              .map(|n| n.trim()
                                        .parse()
                                        .unwrap())
                              .collect();

    /*
    println!("Draws : {:?}\n", draws);
    */

    let (winner, last_draw) = bingo(&mut boards, &draws);
    /*
    println!("Winner index : {}", winner);
    println!("Last Draw : {}\n", last_draw);
    println!("Winner board:\n{:?}", boards[winner]);
    */
    let sum = sum_unmarked(&boards[winner]);
    let score = sum * last_draw as u32;
    /*
    println!("Sum  : {}", sum);
    */
    println!("Answer to Part One : {}", score);
}

fn get_next_board(lines: &mut Vec<String>) -> Board {

    let mut board: Board = [[Cell::new(); 5]; 5];  /* = [[0; 5]; 5]; */

    for i in 0..5 {
        let mut line;

        loop {
            line = lines.pop().unwrap();
            if !line.is_empty() { break; }
        }

        for (j, num) in line.split_ascii_whitespace().enumerate() {
            board[i][j].num = num.parse().unwrap();
        }
    }

    board
}

fn bingo(boards: &mut Vec<Board>, draws: &Vec<u8>) -> (usize, u8) {

    for draw in draws {
        mark(boards, *draw);

        for (idx, board) in boards.iter().enumerate() {
            if has_won(board) == true {
                return (idx, *draw);
            }
        }
    }

    (usize::MAX, u8::MAX)
}

fn mark(boards: &mut Vec<Board>, draw: u8) {

    for board in boards {
        for i in 0..5 {
            for j in 0..5 {
                if board[i][j].num == draw {
                    board[i][j].marked = true;
                }
            }
        }
    }
}

fn has_won(board: &Board) -> bool {

    for row in 0..5 {
        let mut all_marked = true;

        for col in 0..5 {
            if board[row][col].marked == false {
                all_marked = false;
                break;
            }
        }

        if all_marked == true {
            return true;
        }
    }

    for col in 0..5 {
        let mut all_marked = true;

        for row in 0..5 {
            if board[row][col].marked == false {
                all_marked = false;
                break;
            }
        }

        if all_marked == true {
            return true;
        }
    }

    false
}

fn sum_unmarked(board: &Board) -> u32 {
    let mut sum = 0u32;

    for row in 0..5 {
        for col in 0..5 {
            if board[row][col].marked == false {
                sum += board[row][col].num as u32;
            }
        }
    }

    sum
}
