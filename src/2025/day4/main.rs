use std::fs;

const NEIGHBOR_OFFSETS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

fn main() {
    let mut grid: Vec<Vec<char>> = fs::read_to_string("src/2025/day4/input.txt")
        .expect("Could not read file")
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    loop {
        let mut changed = false;

        for x in 0..rows {
            for y in 0..cols {
                if grid[x][y] == '@' && cheak_around(&grid, x, y, rows, cols) < 4 {
                    grid[x][y] = '.';
                    count += 1;
                    changed = true;
                }
            }
        }

        if !changed {
            break;
        }
    }

    dbg!(&count);
}

fn cheak_around(grid: &Vec<Vec<char>>, x: usize, y: usize, rows: usize, cols: usize) -> u8 {
    NEIGHBOR_OFFSETS
        .iter()
        .filter(|&(dx, dy)| {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            nx >= 0
                && nx < rows as isize
                && ny >= 0
                && ny < cols as isize
                && grid[nx as usize][ny as usize] != '.'
        })
        .count() as u8
}
