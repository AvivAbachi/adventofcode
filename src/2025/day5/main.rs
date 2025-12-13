use rayon::prelude::*;
use std::fs;

fn main() {
    let text = fs::read_to_string("src/2025/day5/input.txt").expect("Could not read file");
    let (range_text, items_text) = text.split_once("\r\n\r\n").expect("Invalid input format");

    let mut ranges: Vec<Range> = range_text
        .lines()
        .map(|l| Range::new(l.to_string()))
        .collect();
    ranges.sort_by_key(|r| r.min);

    let freash_item: usize = items_text
        .par_lines()
        .map(|line| {
            let n: u64 = line.parse().expect("Invalid number in items list");

            ranges.iter().any(|r| r.in_range(n)) as usize
        })
        .sum();

    dbg!(freash_item);

    let mut merged_ranges = vec![];
    let mut iter = ranges.iter();

    if let Some(mut current) = iter.next().copied() {
        for &r in iter {
            if current.max >= r.min {
                current.max = current.max.max(r.max);
            } else {
                merged_ranges.push(current);
                current = r;
            }
        }
        merged_ranges.push(current);
    }

    let total_items: u64 = merged_ranges.iter().map(|r| r.get_size()).sum();
    dbg!(total_items);
}

#[derive(Debug, Clone, Copy)]
struct Range {
    min: u64,
    max: u64,
}

impl Range {
    fn new(string: String) -> Self {
        if let Some((min, max)) = string.split_once("-") {
            let min = min.parse().expect("");
            let max = max.parse().expect("");
            Range { min, max }
        } else {
            Range { min: 0, max: 0 }
        }
    }

    fn in_range(&self, number: u64) -> bool {
        number >= self.min && number <= self.max
    }

    fn get_size(&self) -> u64 {
        &self.max - &self.min + 1
    }
}
