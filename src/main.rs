use std::fs;

fn main() {
    let text = fs::read_to_string("src/2025/day1/input.txt").expect("Could not read file");
    dbg!(text);
}
