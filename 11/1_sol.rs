fn main() {
    let mut stones: Vec<u64> = vec![64599, 31, 674832, 2659361, 1, 0, 8867, 321];
    for _ in 0..25 {
        let mut new_stones = Vec::with_capacity(stones.len());
        for stone in stones.into_iter() {
            if stone == 0 {
                new_stones.push(1);
            } else if let digits = stone.ilog10() + 1
                && digits % 2 == 0
            {
                let a = stone / 10u64.pow(digits / 2);
                let b = stone - a * 10u64.pow(digits / 2);
                new_stones.push(a);
                new_stones.push(b);
            } else {
                new_stones.push(2024 * stone);
            }
        }
        stones = new_stones;
    }
    println!("{}", stones.len());
}
