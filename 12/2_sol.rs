use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Plant {
    NotVisited(char),
    Visited(char),
}

impl Plant {
    fn get_char(&self) -> char {
        match self {
            Self::NotVisited(c) => *c,
            Self::Visited(c) => *c,
        }
    }

    fn is_visited(&self) -> bool {
        match self {
            Self::NotVisited(_) => false,
            Self::Visited(_) => true,
        }
    }
}

#[derive(Debug)]
struct Region {
    sides: u32,
    area: u32,
}

impl Region {
    fn new(sides: u32, area: u32) -> Self {
        Self { sides, area }
    }

    fn cost(&self) -> u32 {
        self.sides * self.area
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
}

fn parse_garden(path: &str) -> Vec<Vec<Plant>> {
    let lines = {
        let file = File::open(path).expect("Could not open file");
        let buf_reader = BufReader::new(file);
        buf_reader.lines()
    };

    let mut res = Vec::new();

    for line in lines {
        let line = line.expect("Line is fucked up :/");
        res.push(line.chars().map(|c| Plant::NotVisited(c)).collect());
    }
    res
}

fn flood_fill(garden: &mut Vec<Vec<Plant>>, i: usize, j: usize, plant: Plant) -> Region {
    let m = garden.len();
    let n = garden[0].len();
    let mut q = Vec::new(); // flood fill stack

    let mut area = 0;

    // used for storing adjacent tiles
    // (sometimes coordinates outside of garden when region goes to edge of garden)
    let mut left_perimiter = Vec::new();
    let mut right_perimiter = Vec::new();
    let mut up_perimiter = Vec::new();
    let mut down_perimiter = Vec::new();

    // validate arguments,
    // make sure that the region we are starting has the correct plant type
    // also make sure that the plant type argument is not taken from an explored tile
    if garden[i][j] != plant || plant.is_visited() {
        return Region::new(0, 0);
    }
    q.push(((i as isize, j as isize), Direction::None)); // flood fill stuff

    while let Some(((i, j), direction)) = q.pop() {
        // flood fill stuff
        let valid_tile = i >= 0 && (i as usize) < m && j >= 0 && (j as usize) < n; // tile is within the garden
        if valid_tile && garden[i as usize][j as usize] == plant {
            // tile is of right type
            area += 1;
            garden[i as usize][j as usize] =
                Plant::Visited(garden[i as usize][j as usize].get_char()); // mark as visited

            q.push(((i + 1, j), Direction::Down));
            q.push(((i - 1, j), Direction::Up));
            q.push(((i, j + 1), Direction::Right));
            q.push(((i, j - 1), Direction::Left));

        // if tile has a different kind of plant, it constitutes part of the perimiter
        // but if it is a visited tile of correct plant it does not.
        // "adjacent tiles" not in the garden are also part of the perimiter
        } else if !valid_tile
            || (valid_tile && garden[i as usize][j as usize].get_char() != plant.get_char())
        {
            // insert perimiter
            match direction {
                Direction::Left => {
                    left_perimiter.push((i, j));
                }
                Direction::Right => {
                    right_perimiter.push((i, j));
                }
                Direction::Up => {
                    up_perimiter.push((i, j));
                }
                Direction::Down => {
                    down_perimiter.push((i, j));
                }
                Direction::None => unreachable!(),
            }
        }
    }

    // we keep track of the direction from which the perimiter was reached
    // in order to count some tiles multiple times.
    let mut perimiters = [
        left_perimiter,
        right_perimiter,
        up_perimiter,
        down_perimiter,
    ];
    // number of sides is initially equal to number of boundary tiles
    let mut sides: u32 = perimiters[..].iter().map(|v| v.len() as u32).sum();

    for perimiter in perimiters.iter_mut() {
        // for each tile with the same i and adjecent j, subtract number of sides by one
        // that is, both those are grouped together in one side
        perimiter.sort_by_key(|(_, j)| *j);
        perimiter.sort_by_key(|(i, _)| *i);
        for w in perimiter[..].windows(2) {
            let (i1, j1) = w[0];
            let (i2, j2) = w[1];

            if i1 == i2 && (j2 - j1 == 1) {
                sides -= 1;
            }
        }

        // same thing for equal j with adjacent i's.
        perimiter.sort_by_key(|(_, j)| *j);
        for w in perimiter[..].windows(2) {
            let (i1, j1) = w[0];
            let (i2, j2) = w[1];

            if j1 == j2 && (i2 - i1 == 1) {
                sides -= 1;
            }
        }
    }

    Region::new(sides, area)
}

fn main() {
    let mut garden = parse_garden("./input.txt");
    let m = garden.len();
    let n = garden[0].len();

    let mut regions = Vec::new();

    for i in 0..m {
        for j in 0..n {
            let plant = garden[i][j];
            if !plant.is_visited() {
                regions.push(flood_fill(&mut garden, i, j, plant));
            }
        }
    }

    let s: u64 = regions.into_iter().map(|r| r.cost() as u64).sum();
    println!("{}", s);
}
