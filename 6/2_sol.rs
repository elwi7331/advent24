use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
enum Tile {
    Obstacle,
    Space,
    Visited([bool; 4]),
}

#[derive(Copy, Clone)]
enum Direction {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}

#[derive(Copy, Clone)]
struct Guard {
    direction: Direction,
    i: usize,
    j: usize,
}

impl Guard {
    fn new(direction: Direction, i: usize, j: usize) -> Self {
        Self { direction, i, j }
    }

    fn rotate(&mut self) {
        self.direction.rotate();
    }

    fn get_coordinates(&self) -> (usize, usize) {
        (self.i, self.j)
    }

    fn get_direction(&self) -> Direction {
        self.direction
    }

    fn step(&mut self, m: usize, n: usize) -> Result<(), ()> {
        match self.direction {
            Direction::Left if self.j > 0 => {
                self.j -= 1;
                Ok(())
            }
            Direction::Right if self.j < n - 1 => {
                self.j += 1;
                Ok(())
            }
            Direction::Up if self.i > 0 => {
                self.i -= 1;
                Ok(())
            }
            Direction::Down if self.i < m - 1 => {
                self.i += 1;
                Ok(())
            }
            _ => Err(()),
        }
    }
}

impl Direction {
    fn rotate(&mut self) {
        *self = match self {
            Self::Left => Self::Up,
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
        };
    }
}

fn get_board(path: &str) -> (Vec<Vec<Tile>>, Option<Guard>) {
    let lines = {
        let file = File::open(path).expect("Could not open file");
        let buf_reader = BufReader::new(file);
        buf_reader.lines()
    };

    let mut board: Vec<Vec<Tile>> = Vec::new();
    let mut guard: Option<Guard> = None;

    for (i, line) in lines.into_iter().enumerate() {
        let line = line.expect("Line is sad");
        let mut row = Vec::new();

        for (j, c) in line.chars().enumerate() {
            row.push(match c {
                '#' => Tile::Obstacle,
                '.' => Tile::Space,
                '<' => {
                    guard = Some(Guard::new(Direction::Left, i, j));
                    Tile::Visited([true, false, false, false])
                }
                '>' => {
                    guard = Some(Guard::new(Direction::Right, i, j));
                    Tile::Visited([false, true, false, false])
                }
                '^' => {
                    guard = Some(Guard::new(Direction::Up, i, j));
                    Tile::Visited([false, false, true, false])
                }
                'v' => {
                    guard = Some(Guard::new(Direction::Down, i, j));
                    Tile::Visited([false, false, false, true])
                }
                _ => panic!("Invalid input"),
            })
        }
        board.push(row);
    }
    (board, guard)
}

fn board_terminates(mut board: Vec<Vec<Tile>>, mut guard: Guard) -> bool {
    let m = board.len();
    let n = board[0].len();

    loop {
        let mut new = guard.clone();
        if new.step(m, n).is_err() {
            // guard went out of bounds
            return true;
        }

        let (i, j) = new.get_coordinates();
        match board[i][j] {
            Tile::Obstacle => {
                guard.rotate();
            }
            Tile::Space => {
                let mut visited = [false; 4];
                visited[new.get_direction() as usize] = true;
                board[i][j] = Tile::Visited(visited);
                guard = new;
            }
            Tile::Visited(ref mut visited) => {
                if visited[new.get_direction() as usize] {
                    return false;
                }
                visited[new.get_direction() as usize] = true;
                guard = new;
            }
        }
    }
}

fn main() {
    if let (board, Some(guard)) = get_board("./input.txt") {
        let m = board.len();
        let n = board[0].len();

        let mut res = 0;

        return;
        for i in 0..m {
            for j in 0..n {
                match board[i][j] {
                    Tile::Space => (),
                    _ => {
                        continue;
                    }
                }

                let mut board = board.clone();
                board[i][j] = Tile::Obstacle;
                if !board_terminates(board, guard) {
                    res += 1;
                }
            }
        }
        println!("res: {res}");
    }
}
