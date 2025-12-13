use rayon::prelude::*;
use std::{char, fs};
fn main() {
    let text = fs::read_to_string("src/2025/day6/input.txt").unwrap();
    let text: Vec<Vec<char>> = text
        .par_lines()
        .map(|line| line.chars().into_iter().rev().collect())
        .collect();
    let rows = text.len();
    let cols = text[0].len();

    let mut res: Vec<u64> = vec![];
    let mut bank: Vec<u64> = vec![];

    for col in 0..cols {
        let mut num_str = String::new();

        for row in 0..rows {
            let ch = text[row][col];
            match ch {
                ' ' | '\0' => {}
                '+' | '*' => {
                    bank.push(num_str.parse().unwrap());

                    if let Some(value) = match ch {
                        '+' => Some(bank.iter().sum()),
                        '*' => Some(bank.iter().product()),
                        _ => None,
                    } {
                        res.push(value);
                    }
                    bank.clear();
                    num_str.clear();
                }
                _ => num_str.push(ch),
            }
        }

        if !num_str.is_empty() {
            bank.push(num_str.parse().unwrap());
        };
    }
    let t: u64 = res.iter().sum();
    dbg!(t);
}
