use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy, Debug)]
enum Token {
    Numeral(u16),
    Comma,
    LeftParen,
    RightParen,
    WhiteSpace,
    Mul,
}

fn tokenize(s: &[char]) -> Vec<Token> {
    let mut s = s;
    let mut tokens = Vec::new();

    while s.len() > 0 {
        tokens.push(match s {
            ['(', ..] => {
                s = &s[1..];
                Token::LeftParen
            }
            [')', ..] => {
                s = &s[1..];
                Token::RightParen
            }
            [',', ..] => {
                s = &s[1..];
                Token::Comma
            }
            ['m', 'u', 'l', ..] => {
                s = &s[3..];
                Token::Mul
            }
            [a @ '1'..='9', b @ '0'..='9', c @ '0'..='9', ..] => {
                s = &s[3..];
                Token::Numeral(
                    (a.to_digit(10).unwrap() * 100
                        + b.to_digit(10).unwrap() * 10
                        + c.to_digit(10).unwrap()) as u16,
                )
            }
            [a @ '1'..='9', b @ '0'..='9', ..] => {
                s = &s[2..];
                Token::Numeral((a.to_digit(10).unwrap() * 10 + b.to_digit(10).unwrap()) as u16)
            }
            [a @ '1'..='9', ..] => {
                s = &s[1..];
                Token::Numeral((a.to_digit(10).unwrap()) as u16)
            }
            [_, ..] => {
                s = &s[1..];
                Token::WhiteSpace
            }
            [] => unreachable!(),
        });
    }
    tokens
}

fn execute(tokens: &[Token]) -> usize {
    let mut tokens = tokens;
    let mut sum = 0;

    while tokens.len() > 0 {
        match tokens {
            [Token::Mul, Token::LeftParen, Token::Numeral(a), Token::Comma, Token::Numeral(b), Token::RightParen, ..] =>
            {
                sum += *a as usize * *b as usize;
                tokens = &tokens[6..];
            }
            [_, ..] => {
                tokens = &tokens[1..];
            }
            [] => unreachable!(),
        }
    }
    sum
}

fn main() {
    let mut f = File::open("./input.txt").expect("Could not open file");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("Could not read file");
    let chars: Vec<char> = buffer.chars().collect();

    let tokens = tokenize(&chars);
    let res = execute(&tokens);

    println!("sum: {res}");
}
