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

fn main() {
    let mut res: usize = 0;
    for line in lines_from_file("./input.txt").expect("Could not open file") {
        let line: Vec<isize> = line
            .expect("Line is fucked up")
            .split_whitespace()
            .map(|x| x.parse::<isize>().expect("Failed to parse"))
            .collect();

        if (&line)
            .windows(2)
            .map(|s| (s[0], s[1]))
            .all(|(a, b)| 1 <= a - b && a - b <= 3)
            || (&line)
                .windows(2)
                .map(|s| (s[0], s[1]))
                .all(|(a, b)| 1 <= b - a && b - a <= 3)
        {
            res += 1;
        }
    }
    println!("res: {res}");
}
