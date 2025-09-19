use std::fs::File;
use std::io::{BufRead, BufReader};

fn in_word(i: usize, j: usize, board: &Vec<Vec<char>>) -> u8 {
    let m = board.len();
    let n = board[0].len();

    let space_down = 4 + i <= m;
    let space_up = i >= 3;
    let space_right = 4 + j <= n;
    let space_left = j >= 3;

    let mut occurences = 0;

    if space_down {
        // down
        if board[i][j] == 'X'
            && board[i + 1][j] == 'M'
            && board[i + 2][j] == 'A'
            && board[i + 3][j] == 'S'
        {
            occurences += 1;
        }
    }
    if space_up {
        // up
        if board[i][j] == 'X'
            && board[i - 1][j] == 'M'
            && board[i - 2][j] == 'A'
            && board[i - 3][j] == 'S'
        {
            occurences += 1;
        }
    }
    if space_right {
        // right
        if board[i][j] == 'X'
            && board[i][j + 1] == 'M'
            && board[i][j + 2] == 'A'
            && board[i][j + 3] == 'S'
        {
            occurences += 1;
        }
    }
    if space_left {
        // left
        if board[i][j] == 'X'
            && board[i][j - 1] == 'M'
            && board[i][j - 2] == 'A'
            && board[i][j - 3] == 'S'
        {
            occurences += 1;
        }
    }
    if space_up && space_left {
        if board[i][j] == 'X'
            && board[i - 1][j - 1] == 'M'
            && board[i - 2][j - 2] == 'A'
            && board[i - 3][j - 3] == 'S'
        {
            occurences += 1;
        }
    }
    if space_up && space_right {
        if board[i][j] == 'X'
            && board[i - 1][j + 1] == 'M'
            && board[i - 2][j + 2] == 'A'
            && board[i - 3][j + 3] == 'S'
        {
            occurences += 1;
        }
    }
    if space_down && space_left {
        if board[i][j] == 'X'
            && board[i + 1][j - 1] == 'M'
            && board[i + 2][j - 2] == 'A'
            && board[i + 3][j - 3] == 'S'
        {
            occurences += 1;
        }
    }
    if space_down && space_right {
        if board[i][j] == 'X'
            && board[i + 1][j + 1] == 'M'
            && board[i + 2][j + 2] == 'A'
            && board[i + 3][j + 3] == 'S'
        {
            occurences += 1;
        }
    }
    occurences
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
            if board[i][j] == 'X' {
                res += in_word(i, j, &board) as usize;
            }
        }
    }
    println!("res: {res}");
}
