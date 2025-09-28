use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::{HashMap, HashSet};

fn main() {
    let lines = {
        let file = File::open("./input.txt").expect("Could not open file");
        let buf_reader = BufReader::new(file);
        buf_reader.lines()
    };

    let mut m = 0;
    let mut n: Option<usize> = None;
    let mut frequencies: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for (i, line) in lines.into_iter().enumerate() {
        let line = line.expect("Could not parse line");

        if n.is_none() {
            // count n once
            n = Some(line.clone().chars().count());
        }

        for (j, frequency) in line.chars().enumerate().filter(|(_i, c)| *c != '.') {
            if let Some(antennas) = frequencies.get_mut(&frequency) {
                antennas.push((i as isize, j as isize));
            } else {
                frequencies.insert(frequency, vec![(i as isize, j as isize)]);
            }
        }
        m += 1;
    }

    let n = n.unwrap();

    let mut antinodes = HashSet::new();
    for (_frequency, antennas) in frequencies.iter() {
        for (lo, (i_1, j_1)) in antennas.iter().enumerate() {
            for (i_2, j_2) in antennas[lo + 1..].iter() {
                let (i_3, j_3) = (i_1 + (i_1 - i_2), j_1 + (j_1 - j_2));
                let (i_4, j_4) = (i_2 + (i_2 - i_1), j_2 + (j_2 - j_1));

                if i_3 >= 0 && j_3 >= 0 && (i_3 as usize) < m && (j_3 as usize) < n {
                    antinodes.insert((i_3, j_3));
                }
                if i_4 >= 0 && j_4 >= 0 && (i_4 as usize) < m && (j_4 as usize) < n {
                    antinodes.insert((i_4, j_4));
                }
            }
        }
    }

    println!("res: {}", antinodes.len());
}
