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
    perimiter: u32,
    area: u32,
}

impl Region {
    fn new(perimiter: u32, area: u32) -> Self {
        Self { perimiter, area }
    }
    
    fn cost(&self) -> u32 {
        self.perimiter * self.area
    }
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
    let mut q = Vec::new();
    
    let mut area = 0;
    let mut perimiter = 0;
    
    if garden[i][j] != plant || plant.is_visited() {
        return Region::new(0, 0);
    }
    q.push((i as isize, j as isize));
    
    while let Some((i, j)) = q.pop() {
        if i >= 0 && (i as usize) < m && j >= 0 && (j as usize) < n {
            
            if garden[i as usize][j as usize] == plant {
                area += 1;
                garden[i as usize][j as usize] = Plant::Visited(garden[i as usize][j as usize].get_char());
                
                q.push((i+1, j));
                q.push((i-1, j));
                q.push((i, j+1));
                q.push((i, j-1));
            } else if garden[i as usize][j as usize].get_char() != plant.get_char() {
                perimiter += 1;
            }
        } else {
            perimiter += 1;
        }
    }
    Region::new(perimiter, area)
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