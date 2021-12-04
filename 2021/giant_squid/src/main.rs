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

    let draws: Vec<u8> = lines.first()
                              .unwrap()
                              .split(',')
                              .map(|n| n.trim()
                                        .parse()
                                        .unwrap())
                              .collect();

    let (winner, last_draw) = bingo(&mut boards, &draws);
    let sum = sum_unmarked(&boards[winner]);
    let score = sum * last_draw as u32;
    println!("Answer to Part One : {}", score);

    reset_boards(&mut boards);

    let (winner, last_draw) = bingo_lastwin(&mut boards, &draws);
    let sum = sum_unmarked(&boards[winner]);
    let score = sum * last_draw as u32;
    println!("Answer to Part Two : {}", score);
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

fn bingo_lastwin(boards: &mut Vec<Board>, draws: &Vec<u8>) -> (usize, u8) {

    let len = boards.len();
    let mut winners = vec![false; len];

    for draw in draws {
        mark(boards, *draw);

        for (idx, board) in boards.iter().enumerate() {

            if winners[idx] == false && has_won(board) == true {
                winners[idx] = true;

                /* Did every board win? */
                let mut all_won = true;
                for i in 0..len {
                    if winners[i] == false {
                        all_won = false;
                        break;
                    }
                }

                if all_won == true {
                    return (idx, *draw);
                }
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

fn reset_boards(boards: &mut Vec<Board>) {
    for board in boards {
        for row in 0..5 {
            for col in 0..5 {
                board[row][col].marked = false;
            }
        }
    }
}
