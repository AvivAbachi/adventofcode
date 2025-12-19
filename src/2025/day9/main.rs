use rayon::prelude::*;
use std::fs;

type Point = (u64, u64);

fn main() {
    let text = fs::read_to_string("src/2025/day9/demo.txt").expect("Could not read file");
    let points = text
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let parse = |v: Option<&str>| v.unwrap().parse::<u64>().unwrap();

            (parse(parts.next()), parse(parts.next()))
        })
        .collect::<Vec<Point>>();

    let mut area = (0..points.len())
        .into_par_iter()
        .flat_map(|i| {
            (i + 1..points.len())
                .map(|j| (i, j, get_area(points[i], points[j])))
                .collect::<Vec<(usize, usize, u64)>>()
        })
        .collect::<Vec<(usize, usize, u64)>>();
    area.par_sort_by_key(|v| v.2);
    dbg!(area.last().unwrap().2);
}

fn get_area(a: Point, b: Point) -> u64 {
    (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
}
