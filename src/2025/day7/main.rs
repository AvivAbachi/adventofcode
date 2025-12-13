use rayon::prelude::*;
use std::{
    fs,
    sync::atomic::{AtomicU64, Ordering},
};
fn main() {
    let spliter_counter = AtomicU64::new(0);
    let grid = fs::read_to_string("src/2025/day7/input2.txt").expect("Could not read file");
    let mut grid: Vec<Vec<char>> = grid.par_lines().map(|l| l.chars().collect()).collect();

    let start_y = grid[0].iter().position(|&ch| ch == 'S').unwrap();
    move_on_grid(&mut grid, (1, start_y), &spliter_counter);
    // print_grid(&grid);

    dbg!(spliter_counter.load(Ordering::Relaxed));
}

fn move_on_grid(grid: &mut [Vec<char>], (x, y): (usize, usize), spliter_counter: &AtomicU64) {
    match grid[x][y] {
        '^' => {
            spliter_counter.fetch_add(1, Ordering::Relaxed);
            move_on_grid(grid, (x, y - 1), spliter_counter);
            move_on_grid(grid, (x, y + 1), spliter_counter)
        }
        '.' => {
            // print_grid(grid);

            grid[x][y] = '|';
            if x + 1 < grid.len() {
                move_on_grid(grid, (x + 1, y), spliter_counter)
            }
        }
        _ => {}
    }
}

fn print_grid(grid: &[Vec<char>]) {
    // thread::sleep(Duration::new(0, 100000000));

    let grid: Vec<String> = grid
        .iter()
        .map(|char_vec| char_vec.iter().collect::<String>())
        .collect();
    dbg!(grid);
}
