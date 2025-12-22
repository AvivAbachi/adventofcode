use rayon::prelude::*;
use std::fs;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Eage(Point, Point);

fn main() {
    let text = fs::read_to_string("src/2025/day9/input.txt").expect("Could not read file");

    let points = text
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let parse = |v: Option<&str>| v.unwrap().parse::<usize>().unwrap();
            Point {
                x: parse(parts.next()),
                y: parse(parts.next()),
            }
        })
        .collect::<Vec<Point>>();

    let mut boxs = (0..points.len())
        .into_par_iter()
        .flat_map(|i| {
            (i + 1..points.len())
                .map(|j| (i, j, get_area(points[i], points[j])))
                .collect::<Vec<(usize, usize, usize)>>()
        })
        .collect::<Vec<(usize, usize, usize)>>();

    boxs.par_sort_unstable_by_key(|v| v.2);

    dbg!(&boxs.last().unwrap().2); // part 1

    let mut v_edges = vec![];
    let mut h_edges = vec![];

    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1).rem_euclid(points.len())];
        if p1.x == p2.x {
            v_edges.push(Eage(p1.min(p2), p2.max(p1)));
        } else {
            h_edges.push(Eage(p1.min(p2), p2.max(p1)));
        }
    }

    let boxs: Vec<(usize, usize, usize)> = boxs
        .into_par_iter()
        .filter(|&(i, j, _)| {
            let p1 = points[i];
            let p3 = points[j];
            let p2 = Point { x: p3.x, y: p1.y };
            let p4 = Point { x: p1.x, y: p3.y };

            if ![p1, p2, p3, p4]
                .par_iter()
                .all(|&p| point_in_polygon(p, &v_edges))
            {
                return false;
            }

            [Eage(p1, p2), Eage(p2, p3), Eage(p3, p4), Eage(p1, p4)]
                .par_iter()
                .all(|&e| edge_in_polygon(e, &h_edges, &v_edges))
        })
        .collect();

    dbg!(&boxs.last().unwrap().2); // part 2
}

fn get_area(a: Point, b: Point) -> usize {
    (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
}

fn point_in_polygon(p: Point, v_edges: &[Eage]) -> bool {
    if v_edges
        .par_iter()
        .any(|e| p.x == e.0.x && (e.0.y..=e.1.y).contains(&p.y))
    {
        return true;
    }

    let count = v_edges
        .par_iter()
        .filter(|&e| {
            let y1 = e.0.y;
            let y2 = e.1.y;

            ((y1 <= p.y && p.y < y2) || (y2 <= p.y && p.y < y1)) && p.x < e.0.x
        })
        .count();

    count % 2 == 1
}

fn edge_in_polygon(e: Eage, h_edges: &[Eage], v_edges: &[Eage]) -> bool {
    let is_horizontal = e.0.y == e.1.y;

    if is_horizontal {
        let y = e.0.y;
        let x1 = e.0.x.min(e.1.x);
        let x2 = e.0.x.max(e.1.x);

        !v_edges.par_iter().any(|v| {
            let x = v.0.x;
            let y1 = v.0.y.min(v.1.y);
            let y2 = v.0.y.max(v.1.y);

            x1 < x && x < x2 && y1 < y && y < y2
        })
    } else {
        let x = e.0.x;
        let y1 = e.0.y.min(e.1.y);
        let y2 = e.0.y.max(e.1.y);

        !h_edges.par_iter().any(|h| {
            let y = h.0.y;
            let x1 = h.0.x.min(h.1.x);
            let x2 = h.0.x.max(h.1.x);
            y1 < y && y < y2 && x1 < x && x < x2
        })
    }
}
