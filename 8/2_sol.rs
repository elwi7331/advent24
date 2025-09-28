use std::fs::File;
use std::io::{BufRead, BufReader};

use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::mem::swap;

pub fn gcd(mut u: usize, mut v: usize) -> usize {
    // Base cases: gcd(n, 0) = gcd(0, n) = n
    if u == 0 {
        return v;
    } else if v == 0 {
        return u;
    }

    // Using identities 2 and 3:
    // gcd(2ⁱ u, 2ʲ v) = 2ᵏ gcd(u, v) with u, v odd and k = min(i, j)
    // 2ᵏ is the greatest power of two that divides both 2ⁱ u and 2ʲ v
    let i = u.trailing_zeros();
    u >>= i;
    let j = v.trailing_zeros();
    v >>= j;
    let k = min(i, j);

    loop {
        // u and v are odd at the start of the loop
        debug_assert!(u % 2 == 1, "u = {} should be odd", u);
        debug_assert!(v % 2 == 1, "v = {} should be odd", v);

        // Swap if necessary so u ≤ v
        if u > v {
            swap(&mut u, &mut v);
        }

        // Identity 4: gcd(u, v) = gcd(u, v-u) as u ≤ v and u, v are both odd
        v -= u;
        // v is now even

        if v == 0 {
            // Identity 1: gcd(u, 0) = u
            // The shift by k is necessary to add back the 2ᵏ factor that was removed before the loop
            return u << k;
        }

        // Identity 3: gcd(u, 2ʲ v) = gcd(u, v) as u is odd
        v >>= v.trailing_zeros();
    }
}

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
                let i_diff = i_1 - i_2;
                let j_diff = j_1 - j_2;
                let scale = gcd(i_diff.unsigned_abs(), j_diff.unsigned_abs()) as isize;
                let i_diff = i_diff / scale;
                let j_diff = j_diff / scale;

                for x in 0.. {
                    let new_i = i_1 + x * i_diff;
                    let new_j = j_1 + x * j_diff;
                    if !(new_i >= 0 && new_j >= 0 && (new_i as usize) < m && (new_j as usize) < n) {
                        break;
                    }
                    antinodes.insert((new_i, new_j));
                }
                for x in 0.. {
                    let new_i = i_1 - x * i_diff;
                    let new_j = j_1 - x * j_diff;
                    if !(new_i >= 0 && new_j >= 0 && (new_i as usize) < m && (new_j as usize) < n) {
                        break;
                    }
                    antinodes.insert((new_i, new_j));
                }
            }
        }
    }

    println!("res: {}", antinodes.len());
}
