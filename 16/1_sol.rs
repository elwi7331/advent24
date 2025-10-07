use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Copy, Clone, Hash, Debug)]
struct Player {
    direction: Direction,
    i: usize,
    j: usize,
    cost: usize,
}

impl Player {
    fn new(direction: Direction, i: usize, j: usize, cost: usize) -> Self {
        Self {
            direction,
            i,
            j,
            cost,
        }
    }

    fn step(&mut self) {
        (self.i, self.j) = match self.direction {
            Direction::Left => (self.i, self.j - 1),
            Direction::Right => (self.i, self.j + 1),
            Direction::Up => (self.i - 1, self.j),
            Direction::Down => (self.i + 1, self.j),
        };
        self.cost += 1;
    }

    fn next_positions(mut self) -> [Self; 3] {
        let i = self.i;
        let j = self.j;
        let cost = self.cost;

        let rotations = match self.direction {
            Direction::Left | Direction::Right => [
                Self::new(Direction::Up, i, j, cost + 1000),
                Self::new(Direction::Down, i, j, cost + 1000),
            ],
            Direction::Up | Direction::Down => [
                Self::new(Direction::Left, i, j, cost + 1000),
                Self::new(Direction::Right, i, j, cost + 1000),
            ],
        };

        self.step();

        [self, rotations[0], rotations[1]]
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Player {}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum Tile {
    Wall,
    Space,
    Goal,
    Start,
}

impl TryFrom<char> for Tile {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(Self::Wall),
            '.' => Ok(Self::Space),
            'E' => Ok(Self::Goal),
            'S' => Ok(Self::Start),
            _ => Err(()),
        }
    }
}

struct Labyrinth {
    grid: Vec<Vec<(Tile, [bool; 4])>>, // (variant, visited)
    root: Player,
}

impl Labyrinth {
    fn new(grid: Vec<Vec<Tile>>, root: Player) -> Self {
        Self {
            grid: grid
                .into_iter()
                .map(|row| {
                    iter::zip(row.into_iter(), iter::repeat([false; 4]))
                        .collect::<Vec<(Tile, [bool; 4])>>()
                })
                .collect(),
            root: root,
        }
    }

    /// bfs, but instead of queue, use MinHeap with respect to cost
    fn search(&mut self) -> Option<usize> {
        let mut q = BinaryHeap::new();

        let root = self.root;
        self.grid[root.i][root.j].1[root.direction as usize] = true;
        q.push(root);

        while q.len() > 0 {
            let v = q.pop().unwrap();
            let tile = self.grid[v.i][v.j].0;

            match tile {
                Tile::Goal => {
                    return Some(v.cost);
                }
                Tile::Wall => {}
                _ => {
                    for w in v.next_positions().iter() {
                        let (next_i, next_j) = (w.i, w.j);
                        let next_direction = w.direction;
                        // not labeled as explored
                        if !self.grid[next_i][next_j].1[next_direction as usize] {
                            // mark as explored
                            self.grid[next_i][next_j].1[next_direction as usize] = true;
                            q.push(*w);
                        }
                    }
                }
            }
        }
        None
    }
}

fn parse_labyrinth(path: &str) -> Labyrinth {
    let mut lines = {
        let file = File::open(path).expect("Could not open file");
        let buf_reader = BufReader::new(file);
        buf_reader.lines().enumerate()
    };

    let mut player_pos = None;
    let mut grid = Vec::new();

    while let Some((i, Ok(line))) = lines.next() {
        let mut row = Vec::with_capacity(line.len());

        for (j, c) in line.chars().enumerate() {
            match Tile::try_from(c) {
                Ok(t) => {
                    if player_pos.is_none() && t == Tile::Start {
                        player_pos = Some((i, j));
                    }
                    row.push(t);
                }
                Err(..) => {
                    panic!("Encountered invalid character in input: '{}'", c);
                }
            }
        }
        grid.push(row);
    }

    let (player_i, player_j) = player_pos.expect("Could not find player start location in input");
    let player = Player::new(Direction::Right, player_i, player_j, 0);

    Labyrinth::new(grid, player)
}

fn main() {
    let mut labyrinth = parse_labyrinth("./input.txt");
    if let Some(score) = labyrinth.search() {
        println!("{score}");
    }
}
