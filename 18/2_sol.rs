use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::{VecDeque};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    Empty(bool),
    Obstacle,
}

struct Labyrinth {
    l: Vec<Vec<Tile>>
}

impl Labyrinth {
    fn new(l: Vec<Vec<Tile>>) -> Self {
        Self { l }
    }
    
    /// start and goal are (i, j)
    fn search(&mut self, start: (usize, usize), goal: (usize, usize)) -> bool {
        let root = &mut self.l[start.0][start.1];
        assert_eq!(*root, Tile::Empty(false));
        
        let mut q = VecDeque::new();
        *root = Tile::Empty(true);
        q.push_back(start);
        
        while q.len() > 0 {
            let (i, j) = q.pop_front().unwrap();
            if (i, j) == goal {
                return true;
            }
            
            for (new_i, new_j) in [(i as isize + 1, j as isize), (i as isize - 1, j as isize), (i as isize, j as isize + 1), (i as isize, j as isize - 1)] {
                if new_i >= 0 && new_j >= 0 && (new_i as usize) < self.l.len() && (new_j as usize) < self.l[0].len() {
                    let new_i = new_i as usize;
                    let new_j = new_j as usize;
                    
                    let child = &mut self.l[new_i][new_j];
                    if *child == Tile::Empty(false) {
                        *child = Tile::Empty(true);
                        q.push_back((new_i, new_j));
                    }
                }
            }
        }
        false
    }
}

fn parse_labyrinth(path: &str, width: usize, height: usize, obstacles: usize) -> Labyrinth {
    let lines = {
        let file = File::open(path).expect("Could not open file");
        let buf_reader = BufReader::new(file);
        buf_reader.lines()
    };
    
    let mut l = vec![vec![Tile::Empty(false); height]; width];

    for line in lines.take(obstacles) {
        let line = line.unwrap();
        let mut s = line.split(',');
        let j = s.next().unwrap().parse::<usize>().unwrap();
        let i = s.next().unwrap().parse::<usize>().unwrap();
        
        l[i][j] = Tile::Obstacle;
    }
    Labyrinth::new(l)
}

fn main() {
    for i in 1024..{
        let mut lab = parse_labyrinth("./input.txt", 71, 71, i);
        let found_goal = lab.search((0, 0), (70, 70));
        if !found_goal {
            println!("{i}");
            break;
        }
    }
}