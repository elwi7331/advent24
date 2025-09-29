use std::fs::File;
use std::io::Read;

#[derive(Clone, Copy)]
enum Data {
    File(u32),
    Space
}

fn main() {
    let s = {
        let mut s = String::new();
        let mut file = File::open("./input.txt").expect("Could not open file");
        file.read_to_string(&mut s).expect("Could not read file");
        s
    };
    
    let mut data = Vec::new();
    let mut id = 0;
    for (i, c) in s.chars().enumerate() {
        let space = i % 2 == 1;
        let n: u8 = c.to_digit(10).expect("Could not parse digit") as _;
        if space {
            data.append(&mut vec![Data::Space; n as usize]);
        } else {
            data.append(&mut vec![Data::File(id); n as usize]);
            id += 1;
        }
    }
    
    let mut compact = Vec::new();
    let mut lo = 0;
    let mut hi = data.len() - 1;
    
    while lo <= hi {
        match (data[lo], data[hi]) {
            (Data::File(id), _) => {
                compact.push(id);
                lo += 1;
            }
            (Data::Space, Data::File(id)) => {
                compact.push(id);
                lo += 1;
                hi -= 1;
            }
            (Data::Space, Data::Space) => {
                hi -= 1;
            }
        }
    }
    
    let checksum: u64 = compact.into_iter().enumerate().map(|(i, id)| (i as u64)  * (id as u64)).sum();
    println!("res: {checksum}");
}
