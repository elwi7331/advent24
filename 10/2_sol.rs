use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_map(path: &str) -> Vec<Vec<u8>> {
    let lines = {
        let file = File::open(path).expect("Could not open file");
        let buf_reader = BufReader::new(file);
        buf_reader.lines()
    };

    let mut res = Vec::new();

    for line in lines {
        let line = line.expect("Line is fucked up :/");
        res.push(
            line.chars()
                .map(|c| c.to_digit(10).expect("Failed to parse digit") as u8)
                .collect::<Vec<u8>>(),
        );
    }

    res
}

/// return the number of reachable nodes that satisfy condition
fn bfs(map: &Vec<Vec<u8>>, start_i: usize, start_j: usize, cond: impl Fn(u8) -> bool) -> usize {
    let m = map.len();
    let n = map[0].len();

    let mut count = 0;
    let mut q = VecDeque::new();
    q.push_back((start_i, start_j));

    while q.len() > 0 {
        let (i, j) = q.pop_front().unwrap();
        if cond(map[i][j]) {
            count += 1;
        }

        for (new_i, new_j) in [
            (i as isize + 1, j as isize),
            (i as isize - 1, j as isize),
            (i as isize, j as isize + 1),
            (i as isize, j as isize - 1),
        ]
        .iter()
        {
            if *new_i >= 0 && (*new_i as usize) < m && *new_j >= 0 && (*new_j as usize) < n {
                // check if tiles exist
                let new_i = *new_i as usize;
                let new_j = *new_j as usize;

                if map[new_i][new_j] == map[i][j] + 1 {
                    // check slope
                    q.push_back((new_i, new_j));
                }
            }
        }
    }
    count
}

fn main() {
    let map = parse_map("./input.txt");
    let m = map.len();
    let n = map[0].len();

    let mut sum = 0;

    for i in 0..m {
        for j in 0..n {
            if map[i][j] == 0 {
                sum += bfs(&map, i, j, |x| x == 9);
            }
        }
    }

    println!("sum: {sum}");
}
