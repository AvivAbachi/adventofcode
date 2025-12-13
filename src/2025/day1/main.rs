use std::fs;

pub fn main() {
    let text = fs::read_to_string("exercises/00_intro/input.txt").expect("Could not read file");

    let mut knob = Knob::new(50);

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let dir = line.chars().next().unwrap();
        let amount: i32 = line[1..].parse().expect("Invalid number");

        match dir {
            'R' => knob.move_right(amount),
            'L' => knob.move_left(amount),
            _ => panic!("Unknown direction"),
        };
    }

    println!("{:?}", [knob.total()]);
}

#[derive(Debug)]
struct Knob {
    value: i32,
    zero: i32,
    pass: i32,
}

impl Knob {
    fn new(value: i32) -> Knob {
        Knob {
            value,
            pass: 0,
            zero: 0,
        }
    }

    fn move_knob(&mut self, ammont: i32) {
        self.value = (self.value + ammont).rem_euclid(100);
    }

    fn move_left(&mut self, amount: i32) {
        self.pass += amount / 100;
        let amount = amount % 100;
        if amount == 0 {
            return;
        }
        let pre_value = self.value;
        self.move_knob(-amount);
        if self.value > pre_value && pre_value != 0 {
            self.pass += 1;
        }
        if self.value == 0 {
            self.zero += 1;
        }
    }

    fn move_right(&mut self, amount: i32) {
        self.pass += amount / 100;
        let amount = amount % 100;
        if amount == 0 {
            return;
        }
        let pre_value = self.value;
        self.move_knob(amount);
        if self.value < pre_value && self.value != 0 {
            self.pass += 1;
        }
        if self.value == 0 {
            self.zero += 1;
        }
    }

    fn total(&self) -> i32 {
        self.pass + self.zero
    }
}
