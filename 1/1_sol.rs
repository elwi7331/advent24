use std::fs::File;
use std::path::Path;
use std::convert::AsRef;
use std::io::{BufReader, BufRead};
use std::iter::zip;

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
    let (mut left, mut right) = read_file("./input.txt").expect("Could not read inupt");
    left.sort();
    right.sort();
    let res: usize = zip(left.iter(), right.iter()).map(|(a, b)| a.abs_diff(*b)).sum();
    println!("res: {res}");
}
