#[derive(Copy, Clone, Eq, PartialEq)]
enum Bit {
    Unset,
    One,
    Zero,
}

impl Bit {
    fn bits_to_int(bits: &[Self]) -> usize {
        let mut s = 0;
        for bit in bits.iter() {
            s <<= 1;
            s += match *bit {
                Bit::Unset | Bit::Zero => 0,
                Bit::One => 1,
            };
        }
        s
    }

    fn u3_to_bits(i: u8) -> [Bit; 3] {
        assert!(i < 8);
        [
            if i >> 2 == 1 { Bit::One } else { Bit::Zero },
            if (i & 0b010) >> 1 == 1 {
                Bit::One
            } else {
                Bit::Zero
            },
            if i & 0b001 == 1 { Bit::One } else { Bit::Zero },
        ]
    }
}

fn try_trio(bits: &mut [Bit], p: &[u8], i: usize) {
    let n = bits.len();
    let a_lo = n - 3 * i - 3;

    let mut last_copy = [Bit::Unset; 3];
    last_copy.copy_from_slice(&bits[a_lo..=a_lo + 2]);

    for guess in (0..8).map(|g| Bit::u3_to_bits(g)) {
        // for each bit, if they are either unset or equal to guess,
        // otherwise, we can not proceed
        if (last_copy[0] == Bit::Unset || last_copy[0] == guess[0])
            && (last_copy[1] == Bit::Unset || last_copy[1] == guess[1])
            && (last_copy[2] == Bit::Unset || last_copy[2] == guess[2])
        {
            // set last bits
            bits[a_lo..=a_lo + 2].copy_from_slice(&guess);

            let mut a = (Bit::bits_to_int(&guess) as u8) ^ 0b101;
            // it is possible to go too far...
            if a_lo < a as usize {
                /*
                if 0 == p[i] ^ 0b110 ^ a {
                    let ans = Bit::bits_to_int(&bits);
                    println!("true ans: {}", ans);
                }
                */
                a = 0;
            }
            
            let desired_value = Bit::u3_to_bits(p[i] ^ 0b110 ^ a);
            let mut lower_bits_copy = [Bit::Unset; 3];
            lower_bits_copy.copy_from_slice(&bits[a_lo - (a as usize)..=a_lo - (a as usize) + 2]);

            // for each bit, if they are either unset or equal to guess,
            // otherwise, we can not proceed
            if (lower_bits_copy[0] == Bit::Unset || lower_bits_copy[0] == desired_value[0])
                && (lower_bits_copy[1] == Bit::Unset || lower_bits_copy[1] == desired_value[1])
                && (lower_bits_copy[2] == Bit::Unset || lower_bits_copy[2] == desired_value[2])
            {
                // set "shifted" bits
                bits[a_lo - (a as usize)..=a_lo - (a as usize) + 2].copy_from_slice(&desired_value);

                if a_lo == 0 {
                    let ans = Bit::bits_to_int(&bits);
                    println!("{}", ans);
                } else {
                    try_trio(bits, p, i + 1);
                }

                // reset "shifted" bits
                bits[a_lo - (a as usize)..=a_lo - (a as usize) + 2]
                    .copy_from_slice(&lower_bits_copy);
            }

            // reset last bits
            bits[a_lo..=a_lo + 2].copy_from_slice(&last_copy);
        }
    }
}

fn main() {
    const P: [u8; 16] = [2, 4, 1, 5, 7, 5, 1, 6, 0, 3, 4, 0, 5, 5, 3, 0];
    let mut bits = [Bit::Unset; 48];
    try_trio(&mut bits[..], &P, 0)
}
