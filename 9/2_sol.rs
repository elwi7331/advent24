use std::fs::File;
use std::io::Read;

#[derive(Clone, Copy, Debug)]
enum Data {
    File{id: u32, size: u8},
    Space{size: u8}
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
            data.push(Data::Space{size: n});
        } else {
            data.push(Data::File{id: id, size: n});
            id += 1;
        }
    }
    
    let mut i = 0;
    
    while i < data.len() {
        match data[i] {
            Data::File{..} => { i+= 1; }
            Data::Space{size: space_size} => {
                let mut shift_file: Option<Data> = None;
                for other in data[i+1..].iter_mut().rev() {
                    if let Data::File{id: file_id, size: file_size} = *other && file_size <= space_size {
                        shift_file = Some(Data::File{id: file_id, size: file_size});
                        *other = Data::Space{size: file_size};
                        break;
                    }
                }
                
                if let Some(Data::File{id: file_id, size: file_size}) = shift_file {
                    data[i] = Data::File{id: file_id, size: file_size};
                    if space_size - file_size != 0 {
                        data.insert(i+1, Data::Space{size: space_size - file_size});
                    }
                    i += 1;
                } else {
                    i += 1;
                }
            }
        }
    }
    
    let mut sum: u64 = 0;
    let mut i: u64 = 0;
    for d in data.iter() {
        match d {
            Data::Space{size} => {
                i += *size as u64;
            }
            Data::File{id, size} => {
                for _ in 0..*size {
                    sum += i * (*id as u64);
                    i += 1;
                }
            }
        }
    }
    
    println!("{}", sum);
}
