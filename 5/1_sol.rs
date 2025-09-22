use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut lines = {
        let file = File::open("./input.txt").expect("Could not open file");
        let buf_reader = BufReader::new(file);
        buf_reader.lines()
    }
    .into_iter();

    let mut rules: Vec<(u32, u32)> = Vec::new();

    while let Some(Ok(line)) = lines.next()
        && !line.is_empty()
    {
        if let [x, y] = line
            .split('|')
            .map(|i| i.parse::<u32>().expect("Could not parse u32 in rule"))
            .collect::<Vec<u32>>()[..]
        {
            rules.push((x, y));
        } else {
            unreachable!()
        }
    }

    let mut result = 0;

    while let Some(Ok(line)) = lines.next() {
        let line: Vec<u32> = line
            .split(',')
            .map(|i| i.parse::<u32>().expect("Could not parse u32 in update"))
            .collect();

        let mut visited = HashMap::new();
        for (i, y) in line.iter().enumerate() {
            visited.insert(y, i);
        }

        let mut correct = true;
        for (x, y) in rules.iter() {
            if let Some(x_idx) = visited.get(x)
                && let Some(y_idx) = visited.get(y)
                && !(x_idx < y_idx)
            {
                correct = false;
                break;
            }
        }

        if correct {
            let middle = line[line.len() / 2];
            result += middle;
        }
    }
    println!("result: {result}");
}
