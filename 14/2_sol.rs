use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{thread, time};

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn new(x: i32, y: i32, vx: i32, vy: i32) -> Self {
        Self { x, y, vx, vy }
    }

    fn step(&mut self, steps: i32) {
        self.x += self.vx * steps;
        self.y += self.vy * steps;
    }

    fn modulo(x: i32, m: i32) -> i32 {
        ((x % m) + m) % m
    }

    fn get_x(&self, map_width: u32) -> i32 {
        Self::modulo(self.x, map_width as i32)
    }

    fn get_y(&self, map_height: u32) -> i32 {
        Self::modulo(self.y, map_height as i32)
    }
}

fn parse_robots(path: &str) -> Vec<Robot> {
    let lines = {
        let file = File::open(path).expect("Could not open file");
        let buf_reader = BufReader::new(file);
        buf_reader.lines()
    };

    let mut res = Vec::new();

    for line in lines {
        let line = line.expect("Line is fucked up :/");
        if let [p, v] = line.split_whitespace().collect::<Vec<&str>>()[..]
            && let [_, x, y] = p.split(&['=', ','][..]).collect::<Vec<&str>>()[..]
            && let [_, vx, vy] = v.split(&['=', ','][..]).collect::<Vec<&str>>()[..]
        {
            let x = x.parse::<i32>().expect("Failed to parse int");
            let y = y.parse::<i32>().expect("Failed to parse int");
            let vx = vx.parse::<i32>().expect("Failed to parse int");
            let vy = vy.parse::<i32>().expect("Failed to parse int");
            res.push(Robot::new(x, y, vx, vy));
        } else {
            panic!();
        }
    }
    res
}

fn main() {
    let mut robots = parse_robots("./input.txt");

    let width = 101;
    let height = 103;

    for r in robots.iter_mut() {
        r.step(9);
    }

    for i in 1.. {
        let mut map = vec![vec![0; width as usize]; height as usize];
        for r in robots.iter_mut() {
            r.step(101);
            let x = r.get_x(width) as usize;
            let y = r.get_y(height) as usize;
            map[y][x] += 1;
        }

        println!("iteration: {}", i * 101 + 9);
        for y in 0..height as usize {
            print!(" ");
            for x in 0..width as usize {
                if map[y][x] == 0 {
                    print!("  ");
                } else {
                    print!("X ");
                }
            }
            println!();
        }
        println!();
        println!();
        thread::sleep(time::Duration::from_millis(100));
    }
}
