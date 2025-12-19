use rayon::prelude::*;
use std::{
    collections::HashSet,
    fmt::{self, Display},
    fs,
    num::ParseIntError,
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, PartialEq)]
enum ParsePointError {
    BadLen,
    NoValue,
    ParseInt(ParseIntError),
}

impl Point {
    fn distance(&self, point: &Point) -> f64 {
        let dx = self.x - point.x;
        let dy = self.y - point.y;
        let dz = self.z - point.z;
        ((dx * dx + dy * dy + dz * dz) as f64).sqrt()
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');

        let parse = |v: Option<&str>| {
            let v = v.ok_or(ParsePointError::BadLen)?;
            if v.is_empty() {
                return Err(ParsePointError::NoValue);
            }
            v.parse::<i64>().map_err(ParsePointError::ParseInt)
        };

        let x = parse(parts.next())?;
        let y = parse(parts.next())?;
        let z = parse(parts.next())?;

        if parts.next().is_some() {
            return Err(ParsePointError::BadLen);
        }

        Ok(Point { x, y, z })
    }
}

fn main() {
    let text = fs::read_to_string("src/2025/day8/input.txt").expect("Could not read file");
    let points: Vec<Point> = text
        .par_lines()
        .map(|s| Point::from_str(s).unwrap())
        .collect();

    let mut distances = (0..points.len())
        .into_par_iter()
        .flat_map(|i| {
            (i + 1..points.len())
                .map(|j| (i, j, { points[i].distance(&points[j]) }))
                .collect::<Vec<(usize, usize, f64)>>()
        })
        .collect::<Vec<(usize, usize, f64)>>();

    distances.par_sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut connections: Vec<HashSet<usize>> = vec![];
    let mut res = 0;

    // part 1 - demo .take(10) ,part 1 .take(1000), part 2 all
    distances.iter().for_each(|&(p1, p2, _)| {
        let a_inx = connections.par_iter().position_first(|s| s.contains(&p1));
        let b_inx = connections.par_iter().position_first(|s| s.contains(&p2));

        match (a_inx, b_inx) {
            (Some(a), Some(b)) => {
                if let Some((first, last)) = match a.cmp(&b) {
                    std::cmp::Ordering::Greater => Some((b, a)),
                    std::cmp::Ordering::Less => Some((a, b)),
                    std::cmp::Ordering::Equal => None,
                } {
                    let last_set = connections.remove(last);
                    let first_set = &mut connections[first];
                    first_set.extend(last_set);
                }
            }
            (Some(a), None) => {
                connections[a].insert(p2);
            }
            (None, Some(b)) => {
                connections[b].insert(p1);
            }
            (None, None) => {
                connections.push(HashSet::from([p1, p2]));
            }
        }
        if connections[0].len() == points.len() && a_inx != b_inx {
            res = points[p1].x * points[p2].x;
        }
    });

    connections.par_sort_by_key(|a| a.len());
    dbg!(&connections.iter().rev().take(3).fold(1, |a, b| a * b.len())); //part 1
    dbg!(res); // part 2
}
