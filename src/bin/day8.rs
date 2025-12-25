use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

fn main() {
    type Pint: u32;

    let handle = io::stdin().lock().lines();

    let boxes: Vec<Point> = handle
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .into_iter()
        .map(|s| {
            let iv: Vec<u64> = s.splitn(3, ',').map(|n| n.parse().unwrap()).collect();
            Point {
                x: iv[0],
                y: iv[1],
                z: iv[2],
            }
        })
        .collect();
    dbg!(&boxes);

    let dist2 = |a: Point, b: Point| (a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2);

    let mut closest_pair = (boxes[0], boxes[1]);
    let mut best_d2 = u64::MAX;
    let mut cell_size = u64::MAX;
    let mut grid: HashMap<(u64, u64, u64), Vec<Point>> = HashMap::new();
}
