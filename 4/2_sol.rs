use std::fs::File;
use std::io::{BufRead, BufReader};

fn in_word(i: usize, j: usize, board: &Vec<Vec<char>>) -> bool {
    let m = board.len();
    let n = board[0].len();

    if i > 0 && i < m - 1 && j > 0 && j < n - 1 {
        ((board[i - 1][j - 1] == 'M' && board[i + 1][j + 1] == 'S')
            || (board[i - 1][j - 1] == 'S' && board[i + 1][j + 1] == 'M'))
            && ((board[i + 1][j - 1] == 'M' && board[i - 1][j + 1] == 'S')
                || (board[i + 1][j - 1] == 'S' && board[i - 1][j + 1] == 'M'))
    } else {
        false
    }
}

fn main() {
    let f = File::open("./input.txt").expect("Could not open file");
    let reader = BufReader::new(f);

    let mut board: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        board.push(line.unwrap().chars().collect::<Vec<char>>());
    }
    let mut res = 0;
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == 'A' && in_word(i, j, &board) {
                res += 1;
            }
        }
    }
    println!("res: {res}");
}
