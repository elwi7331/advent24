use std::collections::HashMap;

fn main() {
    let mut stones: HashMap<u64, u64> = HashMap::from([(64599, 1), (31, 1), (674832, 1), (2659361, 1), (1, 1), (0, 1), (8867, 1), (321, 1)]);
    // let mut stones: HashMap<u64, u64> = HashMap::from([(125, 1), (17, 1)]);
    for _ in 0..75 {
        let mut new_stones = Vec::new();
        for (stone, v) in stones.iter_mut().filter(|(_, v)| **v != 0) {
            if *stone == 0 {
                new_stones.push((1, *v));
            } else if let digits = stone.ilog10() + 1
                && digits % 2 == 0
            {
                let a = *stone / 10u64.pow(digits / 2);
                let b = *stone - a * 10u64.pow(digits / 2);
                new_stones.push((a, *v));
                new_stones.push((b, *v));
            } else {
                new_stones.push((2024 * *stone, *v));
            }
            *v = 0;
        }
        for (stone, occurences) in new_stones.into_iter() {
            if let Some(v) = stones.get_mut(&stone) {
                *v += occurences;
            } else {
                stones.insert(stone, occurences);
            }
        }
    }
    // for (k, v) in stones.iter() {
    //     for _ in 0..*v {
    //         print!("{} ", k);
    //     }
    // }
    
    let s: u64 = stones.into_values().sum();
    println!("{}", s);
}
