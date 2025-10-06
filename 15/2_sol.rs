use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::rc::Rc;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum ElementType {
    Wall,
    Robot,
    Box,
    Space,
}

#[derive(Copy, Clone)]
struct Element {
    pub variant: ElementType,
    pub height: u8,
    pub width: u8,
}

impl Element {
    /// For Spaces, height and width have to be 1
    fn new(variant: ElementType, height: u8, width: u8) -> Self {
        Self {
            variant,
            height,
            width,
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.variant == ElementType::Box && self.width == 2 && self.height == 1 {
            write!(f, "[]")
        } else {
            let c = match self.variant {
                ElementType::Box => 'O',
                ElementType::Robot => '@',
                ElementType::Wall => '#',
                ElementType::Space => '.',
            };
            let mut s = String::new();
            let line: String = iter::repeat(c).take(self.width as usize).collect();
            for _ in 0..self.height {
                s.push_str(&line);
                s.push('\n');
            }
            s.pop();
            write!(f, "{}", s)
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum StepResult {
    Unable,
    NoEffect,
    Success(usize, usize),
}

struct WareHouse {
    map: HashMap<(usize, usize), Rc<RefCell<(Element, (usize, usize))>>>,
    height: usize,
    width: usize,
}

impl WareHouse {
    fn new(tiles: Vec<(Element, (usize, usize))>, height: usize, width: usize) -> Self {
        let mut map = HashMap::new();
        for (elem, (i, j)) in tiles.into_iter() {
            let cell = Rc::new(RefCell::new((elem, (i, j))));
            for x in 0..elem.width {
                for y in 0..elem.height {
                    map.insert((i + y as usize, j + x as usize), cell.clone());
                }
            }
        }
        Self { map, width, height }
    }

    fn score(&self) -> usize {
        let mut s = 0;
        for ((k_i, k_j), val) in self.map.iter() {
            let (elem, (v_i, v_j)) = *val.borrow();
            if elem.variant == ElementType::Box && *k_i == v_i && *k_j == v_j {
                s += 100 * v_i+ v_j;
            }
        }
        s
    }
    
    fn perform_step(&mut self, i: usize, j: usize, new_i: usize, new_j: usize) {
        let (elem, (i, j)) = *self.map.get(&(i, j)).unwrap().borrow();
        
        let mut coordinates_to_clear: HashSet<(usize, usize)> = HashSet::new();
        for i in i..i+elem.height as usize {
            for j in j..j+elem.width as usize {
                coordinates_to_clear.insert((i, j));
            }
        }

        let mut coordinates_to_fill: HashSet<(usize, usize)> = HashSet::new();
        for i in new_i..new_i+elem.height as usize {
            for j in new_j..new_j+elem.width as usize {
                coordinates_to_fill.insert((i, j));
            }
        }

        for (i, j) in coordinates_to_clear.difference(&coordinates_to_fill) {
            self.map.insert(
                (*i, *j),
                Rc::new(RefCell::new((
                    Element::new(ElementType::Space, 1, 1),
                    (*i, *j),
                ))),
            );
        }

        let cell = Rc::new(RefCell::new((elem, (new_i, new_j))));

        for (i, j) in coordinates_to_fill.iter() {
            self.map.insert((*i, *j), cell.clone());
        }
    }

    fn able_to_step(&self, i: usize, j: usize, direction: Direction, acc: &mut Vec<((usize, usize), StepResult)>) {
        let (elem, (i, j)) = *self.map.get(&(i, j)).unwrap().borrow();
        match elem.variant {
            ElementType::Wall => {
                acc.push(((i, j), StepResult::Unable));
            }
            ElementType::Space => {
                acc.push(((i, j), StepResult::NoEffect));
            }
            _ => {
                let coliding_coordinates: Vec<(usize, usize)> = match direction {
                    // coordinates of adjacent element
                    Direction::Left => (i..i + elem.height as usize).map(|i| (i, j - 1)).collect(),
                    Direction::Right => (i..i + elem.height as usize).map(|i| (i, j + elem.width as usize)).collect(),
                    Direction::Up => (j..j + elem.width as usize).map(|j| (i - 1, j)).collect(),
                    Direction::Down => (j..j + elem.width as usize).map(|j| (i + elem.height as usize, j)).collect(),
                };

                for &(i, j) in coliding_coordinates.iter() {
                    self.able_to_step(i, j, direction, acc);
                }
                
                if acc
                    .iter()
                    .all(|((_, _), r)| *r != StepResult::Unable)
                {
                    let (i_new, j_new) = match direction {
                        Direction::Left => (i, j - 1),
                        Direction::Right => (i, j + 1),
                        Direction::Up => (i - 1, j),
                        Direction::Down => (i + 1, j),
                    };
                    
                    acc.push(((i, j), StepResult::Success(i_new, j_new)));
                }
            }
        }
    }
    
    // Because the boxes we push might no longer line up, we have to check in potentially
    // mutliple "branches" before moving the initial boxes.
    fn step(&mut self, i: usize, j: usize, direction: Direction) -> StepResult {
        let mut study = Vec::new();
        self.able_to_step(i, j, direction, &mut study);
        
        // if we try to move the same Element twice, we get issues
        // because then, it reads 1x1 spaces and will move them into the new element position
        let mut moved = HashSet::new();
        
        if study.iter().any(|((_, _), r)| *r == StepResult::Unable) {
            StepResult::Unable
        } else {
            for ((old_i, old_j), res) in study.iter() {
                if let StepResult::Success(new_i, new_j) = *res && moved.get(&(old_i, old_j)).is_none() {
                    moved.insert((old_i, old_j));
                    self.perform_step(*old_i, *old_j, new_i, new_j);
                }
            }
            study.last().unwrap().1
        }
    }
}

impl fmt::Display for WareHouse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = vec![vec![' '; self.width + 1]; self.height];
        for (k, val) in self.map.iter() {
            let (elem, (i, j)) = *val.borrow();
            if *k == (i, j) {
                for (y, line) in elem.to_string().lines().enumerate() {
                    for (x, c) in line.chars().enumerate() {
                        res[i + y][j + x] = c;
                    }
                }
            }
        }
        write!(
            f,
            "{}",
            res.iter()
                .map(|l| l.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        )
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
    let mut width = None;
    let mut height = 0;
    let mut elements = Vec::new();

    // parse map
    while let Some((i, Ok(line))) = lines.next()
        && line.len() != 0
    {
        if line.len() == 0 {
            break;
        }

        height += 1;
        width = Some(line.len() * 2);

        for (mut j, c) in line.chars().enumerate() {
            j *= 2;
            match c {
                '#' => {
                    elements.push((Element::new(ElementType::Wall, 1, 2), (i, j)));
                }
                '.' => {
                    elements.push((Element::new(ElementType::Space, 1, 1), (i, j)));
                    elements.push((Element::new(ElementType::Space, 1, 1), (i, j+1)));
                }
                'O' => {
                    elements.push((Element::new(ElementType::Box, 1, 2), (i, j)));
                }
                '@' => {
                    robot_i = Some(i);
                    robot_j = Some(j);
                    elements.push((Element::new(ElementType::Robot, 1, 1), (i, j)));
                    elements.push((Element::new(ElementType::Space, 1, 1), (i, j + 1)));
                }
                _ => unreachable!(),
            }
        }
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
        WareHouse::new(elements, height, width.unwrap()),
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
    println!("{}", warehouse);
    println!("{}", warehouse.score());
}
