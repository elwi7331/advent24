use std::convert::AsRef;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

fn lines_from_file<P: AsRef<Path>>(
    path: P,
) -> Result<Lines<BufReader<File>>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}

fn safe_with_exception(line: &[isize], f: impl Fn(isize) -> bool) -> bool {
    let mut lo = 0;
    let mut hi = line.len() - 1;
    while lo < hi {
        let a = line[lo];
        let b = line[lo + 1];
        if f(a - b) {
            lo += 1;
        } else {
            break;
        }
    }
    while lo < hi {
        let b = line[hi];
        let a = line[hi - 1];

        if f(a - b) {
            hi -= 1;
        } else {
            break;
        }
    }

    if lo == hi {
        true
    } else if hi - lo == 1 {
        if lo == 0 {
            true
        } else if hi == line.len() - 1 {
            true
        } else if lo > 0 && f(line[lo - 1] - line[hi]) {
            true
        } else if hi + 1 < line.len() && f(line[lo] - line[hi + 1]) {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn is_valid(s: &[isize]) -> bool {
    s.windows(2).map(|v| v[0] - v[1]).all(|x| 1 <= x && x <= 3)
        || s.windows(2).map(|v| v[1] - v[0]).all(|x| 1 <= x && x <= 3)
}

fn main() {
    let mut res = 0;
    for line in lines_from_file("./input.txt").expect("Could not open file") {
        let line: Vec<isize> = line
            .expect("Line is fucked up")
            .split_whitespace()
            .map(|x| x.parse::<isize>().expect("Failed to parse"))
            .collect();

        if is_valid(&line) {
            res += 1;
        } else {
            for i in 0..line.len() {
                let mut l = line.clone();
                l.remove(i);
                if is_valid(&l) {
                    res += 1;
                    break;
                }
            }
        }
    }
    println!("res: {res}");
}
