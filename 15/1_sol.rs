use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
enum Tile {
    Wall,
    Robot,
    Box,
    Space,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, "{}", match self {
                Self::Wall => "#",
                Self::Robot => "@",
                Self::Box => "O",
                Self::Space => ".",
            }
        )
    }
}

enum StepResult {
    Unable,
    NoEffect,
    Success(usize, usize),
}

struct WareHouse {
    map: Vec<Vec<Tile>>,
}

impl WareHouse {
    fn new(map: Vec<Vec<Tile>>) -> Self {
        Self { map }
    }

    fn score(&self) -> usize {
        let mut s = 0;

        for (i, row) in self.map.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                if let Tile::Box = tile {
                    s += 100 * i + j;
                }
            }
        }
        s
    }

    fn step(&mut self, i: usize, j: usize, direction: Direction) -> StepResult {
        match self.map[i][j] {
            Tile::Wall => StepResult::Unable,
            Tile::Space => StepResult::NoEffect,
            tile_variant @ Tile::Robot | tile_variant @ Tile::Box => {
                let (i1, j1) = match direction {
                    Direction::Left => (i, j - 1),
                    Direction::Right => (i, j + 1),
                    Direction::Up => (i - 1, j),
                    Direction::Down => (i + 1, j),
                };
                match self.step(i1, j1, direction) {
                    StepResult::Unable => StepResult::Unable,
                    StepResult::NoEffect | StepResult::Success(..) => {
                        self.map[i][j] = Tile::Space;
                        self.map[i1][j1] = tile_variant;
                        StepResult::Success(i1, j1)
                    }
                }
            }
        }
    }
}

impl fmt::Display for WareHouse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for row in self.map.iter() {
            for tile in row.iter() {
                s.push_str(&tile.to_string());
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

fn parse_warehouse(path: &str) -> (WareHouse, Vec<Direction>, (usize, usize)) {
    let mut lines = {
        let file = File::open(path).expect("Could not open file");
        let buf_reader = BufReader::new(file);
        buf_reader.lines().enumerate()
    };

    let mut robot_i = None;
    let mut robot_j = None;
    let mut map = Vec::new();

    // parse map
    while let Some((i, Ok(line))) = lines.next() && line.len() != 0 {
        if line.len() == 0 {
            break;
        }

        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            row.push(match c {
                '#' => Tile::Wall,
                '.' => Tile::Space,
                'O' => Tile::Box,
                '@' => {
                    robot_i = Some(i);
                    robot_j = Some(j);
                    Tile::Robot
                }
                _ => unreachable!(),
            });
        }
        map.push(row);
    }

    let mut steps = Vec::new();
    // parse steps
    for (_, line) in lines {
        let line = line.expect("Line is fucked up :/");
        for c in line.chars() {
            steps.push(match c {
                '<' => Direction::Left,
                '>' => Direction::Right,
                '^' => Direction::Up,
                'v' => Direction::Down,
                _ => unreachable!(),
            })
        }
    }

    (
        WareHouse::new(map),
        steps,
        (robot_i.unwrap(), robot_j.unwrap()),
    )
}

fn main() {
    let (mut warehouse, steps, (mut i, mut j)) = parse_warehouse("./input.txt");

    for dir in steps.into_iter() {
        if let StepResult::Success(new_i, new_j) = warehouse.step(i, j, dir) {
            i = new_i;
            j = new_j;
        }
    }

    println!("{}", warehouse.score());
}
