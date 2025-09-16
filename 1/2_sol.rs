use std::fs::File;
use std::path::Path;
use std::convert::AsRef;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn read_file<P: AsRef<Path>>(path: P) -> Result<(Vec<usize>, Vec<usize>), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in buf_reader.lines() {
        let line = line?;
        let mut numerals = line.split_whitespace();
        let a: usize = numerals.next().unwrap().parse()?;
        let b: usize = numerals.next().unwrap().parse()?;
        left.push(a);
        right.push(b);
    }
    Ok((left, right))
}

fn main() {
    let (left, right) = read_file("./input.txt").expect("Could not read input");
    let mut map = HashMap::new();
    for elem in right.into_iter() {
        if let Some(freq) = map.get_mut(&elem) {
            *freq += 1;
        } else {
            map.insert(elem, 1);
        }
    }
    
    let mut sum = 0;
    for elem in left.iter() {
        if let Some(freq) = map.get(elem) {
            sum += *elem * freq;
        }
    }
    println!("sum: {sum}");
}
