use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

type Pos = (usize, usize);

fn main() {
    let mut spliter_counter = 0;
    let grid = fs::read_to_string("src/2025/day7/input.txt").expect("Could not read file");
    let mut grid: Vec<Vec<char>> = grid.par_lines().map(|l| l.chars().collect()).collect();
    let grid2 = grid.clone();
    let start_y = grid[0].iter().position(|&ch| ch == 'S').unwrap();

    let mut qu = VecDeque::new();
    let mut seen = HashSet::new();

    add((1, start_y), &mut qu, &mut seen);

    while !qu.is_empty() {
        if let Some(pos) = qu.pop_front() {
            match grid[pos.0][pos.1] {
                '.' => {
                    grid[pos.0][pos.1] = '|';
                    if pos.0 < grid.len() - 1 {
                        add((pos.0 + 1, pos.1), &mut qu, &mut seen);
                    }
                }
                '^' => {
                    spliter_counter += 1;
                    add((pos.0, pos.1 - 1), &mut qu, &mut seen);
                    add((pos.0, pos.1 + 1), &mut qu, &mut seen);
                }
                _ => {}
            }
        }
    }

    print_grid(&grid);

    dbg!(spliter_counter);

    let mut memo: HashMap<Pos, u64> = HashMap::new();
    dbg!(timeline((1, start_y), &grid2, &mut memo));
}

fn timeline(pos: Pos, grid: &[Vec<char>], memo: &mut HashMap<Pos, u64>) -> u64 {
    if pos.0 >= grid.len() {
        return 1;
    }

    if memo.contains_key(&pos) {
        return memo[&pos];
    }

    let res = match grid[pos.0][pos.1] {
        '.' => timeline((pos.0 + 1, pos.1), grid, memo),
        '^' => timeline((pos.0, pos.1 - 1), grid, memo) + timeline((pos.0, pos.1 + 1), grid, memo),
        _ => 0,
    };

    memo.insert(pos, res);
    res
}

fn add(pos: Pos, qu: &mut VecDeque<Pos>, seen: &mut HashSet<Pos>) {
    if !seen.contains(&pos) {
        qu.push_back(pos);
        seen.insert(pos);
    }
}

fn print_grid(grid: &[Vec<char>]) {
    let grid: Vec<String> = grid
        .iter()
        .map(|char_vec| char_vec.iter().collect::<String>())
        .collect();
    dbg!(grid);
}
