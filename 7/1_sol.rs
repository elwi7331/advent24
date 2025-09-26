use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::ops::{Add, Mul};

fn evaluate_equation(
    terms: &[usize],
    operators: &[fn(usize, usize) -> usize],
) -> Result<usize, ()> {
    if terms.len() != operators.len() + 1 {
        return Err(());
    }

    let mut terms = terms.iter();

    let mut acc: usize = *terms.next().unwrap_or(&0);
    for (term, operator) in zip(terms, operators.iter()) {
        acc = operator(acc, *term);
    }
    Ok(acc)
}

fn try_all(terms: &[usize], operators: &mut Vec<fn(usize, usize) -> usize>, answer: usize) -> bool {
    if operators.len() + 1 == terms.len() {
        return evaluate_equation(terms, operators).expect("Could not evaluate equation") == answer;
    }

    for op in [usize::add, usize::mul].iter() {
        operators.push(*op);
        if try_all(terms, operators, answer) {
            return true;
        }
        operators.pop();
    }
    false
}

fn main() {
    let lines = {
        let file = File::open("./input.txt").expect("Could not open file");
        let buf_reader = BufReader::new(file);
        buf_reader.lines()
    };

    let mut res = 0;

    for line in lines.into_iter() {
        let line = line.expect("Could not parse line");
        let parsed: Vec<usize> = line
            .split(|c: char| c == ':' || c == ' ')
            .filter(|s| s.len() > 0)
            .map(|s| {
                s.parse::<usize>()
                    .expect(&format!("Could not parse integer {s}"))
            })
            .collect();

        let target = parsed[0];
        let terms = &parsed[1..];
        let mut operators = Vec::with_capacity(terms.len() - 1);
        if try_all(terms, &mut operators, target) {
            res += target;
        }
    }
    println!("res: {res}");
}
