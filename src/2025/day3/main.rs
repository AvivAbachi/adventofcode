use std::fs;

fn main() {
    let mut res = vec![];
    fs::read_to_string("src/2025/day3/input.txt")
        .expect("Could not read file")
        .lines()
        .for_each(|line| {
            if !line.is_empty() {
                res.push(max_num(line));
            }
        });

    let sum: u64 = res.iter().sum();
    dbg!(&sum);
}
const SIZE: usize = 12;

fn max_num(str: &str) -> u64 {
    let mut nums = Vec::with_capacity(SIZE);
    let mut to_remove = str.len() - SIZE;

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

    nums.truncate(SIZE);

    let mut res: u64 = 0;
    for num in nums {
        res = res * 10 + num as u64;
    }
    res
}
