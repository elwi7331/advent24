
fn main () {
    let input = [2,4,1,5,7,5,1,6,0,3,4,0,5,5,3,0];
    let n = input.len();
    let lo: usize = 8usize.pow(n as u32-1);
    let hi: usize = 8usize.pow(n as u32);
    
    'outer: for initial_a in lo..=hi {
        let mut a = initial_a;
        let mut b;
        let mut c;
        let mut i = 0;
        
        while a != 0 {
            c = a >> ((a & 0b111) ^ 101);
            a = a >> 3;
            b = (a & 0b111) ^ 011 ^ c;
            if (b & 0b111) != input[i] {
                break;
            } else if i == n-1 {
                println!("answer: {}", initial_a);
                break 'outer;
            }
            i += 1;
        }
        
        if initial_a % 1_000_000_000 == 0 {
            println!("{i}");
        }
    }
}