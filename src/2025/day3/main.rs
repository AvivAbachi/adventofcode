use std::fs;

fn main() {
    let mut res: Vec<u128> = vec![];
    fs::read_to_string("src/2025/day3/input.txt")
        .expect("Could not read file")
        .lines()
        .for_each(|line| {
            if !line.is_empty() {
                res.push(max_num(line));
            }
        });

    let sum: u128 = res.iter().sum();
    dbg!(&sum);
}
const SIZE: usize = 12;

fn max_num(str: &str) -> u128 {
    let mut nums: Vec<u32> = vec![];
    let mut to_remove: usize = str.len() - SIZE;

    for c in str.chars() {
        let n = c.to_digit(10);
        match n {
            Some(n) => {
                while let Some(&last) = nums.last() {
                    if last < n && to_remove > 0 {
                        nums.pop();
                        to_remove -= 1;
                    } else {
                        break;
                    }
                }
                nums.push(n);
            }
            None => continue,
        }
    }
    nums.truncate(12);
    let mut res: u128 = 0;
    for num in nums {
        res = res * 10 + num as u128;
    }
    res
}
