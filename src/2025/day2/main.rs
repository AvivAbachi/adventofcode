use std::{fs, path::Path};

pub fn main() {
    let path = Path::new("src/2025/day2/input.txt");

    let text = fs::read_to_string(path).expect("Could not read file");
    let mut side = false; //false=start, true=end
    let mut start = String::new();
    let mut end = String::new();
    let mut invalid_id: Vec<String> = Vec::new();

    for ch in text.chars() {
        if ch == ',' || ch == ' ' {
            str(&start, &end, &mut invalid_id);

            side = false;
            start.clear();
            end.clear();
            continue;
        }

        if ch == '-' {
            side = true;
            continue;
        }

        if !side {
            start.push(ch);
        } else {
            end.push(ch);
        }
    }
    str(&start, &end, &mut invalid_id);
    dbg!(&invalid_id);

    println!(
        "{:?}",
        invalid_id
            .iter()
            .map(|s| s.parse::<u128>().unwrap())
            .sum::<u128>()
    );
}

fn str(start: &str, end: &str, invalid_id: &mut Vec<String>) {
    let start_num: u128 = start.parse().expect("msg");
    match end.parse() {
        Ok(end_num) => {
            for n in start_num..=end_num {
                let str = n.to_string();
                if seq_str(&str) {
                    invalid_id.push(str);
                }
            }
        }
        Err(_) => {
            if seq_str(&start) {
                invalid_id.push(start.to_string());
            }
        }
    }
}

fn seq_str(string: &str) -> bool {
    let len = string.len();

    for n in 1..=(&len / 2) {
        let mut chunks = Vec::new();

        if &len % n != 0 {
            continue;
        }

        let mut i = 0;
        while i < len {
            let c = &string[i..(i + n)];
            chunks.push(c);
            i += n;
        }
        if chunks.iter().all(|&c| c == chunks[0]) {
            return true;
        }
    }

    false
}
