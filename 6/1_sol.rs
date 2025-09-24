use std::fs::File;
use std::io::{BufRead, BufReader};

enum Tile {
    Obstacle,
    Visited,
    Space,
}

enum Guard {
    Left(usize, usize),
    Right(usize, usize),
    Up(usize, usize),
    Down(usize, usize),
    None,
}

impl Guard {
    fn rotate(&mut self) {
        *self = match self {
            Self::Left(i, j) => Self::Up(*i, *j),
            Self::Up(i, j) => Self::Right(*i, *j),
            Self::Right(i, j) => Self::Down(*i, *j),
            Self::Down(i, j) => Self::Left(*i, *j),
            Self::None => Self::None,
        };
    }

    fn get_coordinates(&self) -> Option<(usize, usize)> {
        match self {
            Self::Left(i, j) => Some((*i, *j)),
            Self::Up(i, j) => Some((*i, *j)),
            Self::Right(i, j) => Some((*i, *j)),
            Self::Down(i, j) => Some((*i, *j)),
            Self::None => None,
        }
    }
}

fn get_board(path: &str) -> (Vec<Vec<Tile>>, Guard) {
    let lines = {
        let file = File::open(path).expect("Could not open file");
        let buf_reader = BufReader::new(file);
        buf_reader.lines()
    };

    let mut board: Vec<Vec<Tile>> = Vec::new();
    let mut guard = Guard::None;

    for (i, line) in lines.into_iter().enumerate() {
        let line = line.expect("Line is sad");
        let mut row = Vec::new();

        for (j, c) in line.chars().enumerate() {
            row.push(match c {
                '.' => Tile::Space,
                '#' => Tile::Obstacle,
                '^' => {
                    guard = Guard::Up(i, j);
                    Tile::Visited
                }
                '>' => {
                    guard = Guard::Right(i, j);
                    Tile::Visited
                }
                'v' => {
                    guard = Guard::Down(i, j);
                    Tile::Visited
                }
                '<' => {
                    guard = Guard::Left(i, j);
                    Tile::Visited
                }
                _ => unreachable!(),
            })
        }
        board.push(row);
    }
    (board, guard)
}

fn main() {
    let (mut board, mut guard) = get_board("./input.txt");
    let m = board.len();
    let n = board[0].len();

    let mut visited_tiles = 1;

    loop {
        let next = match guard {
            Guard::Up(0, _) => {
                break;
            }
            Guard::Up(i, j) => Guard::Up(i - 1, j),
            Guard::Down(i, _) if i == m - 1 => {
                break;
            }
            Guard::Down(i, j) => Guard::Down(i + 1, j),
            Guard::Left(_, 0) => {
                break;
            }
            Guard::Left(i, j) => Guard::Left(i, j - 1),
            Guard::Right(_, j) if j == n - 1 => {
                break;
            }
            Guard::Right(i, j) => Guard::Right(i, j + 1),
            Guard::None => unreachable!(),
        };

        if let Some((i, j)) = next.get_coordinates() {
            match board[i][j] {
                Tile::Obstacle => {
                    guard.rotate();
                }
                Tile::Visited => {
                    guard = next;
                }
                Tile::Space => {
                    board[i][j] = Tile::Visited;
                    visited_tiles += 1;
                    guard = next;
                }
            }
        } else {
            unreachable!()
        }
    }
    println!("{visited_tiles}");
}
