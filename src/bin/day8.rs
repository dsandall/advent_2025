use core::str;
use std::collections::HashSet;
use std::io::{self, Read};

type Point = (u32, u32, u32);

fn main() {
    let num_pairings = 10;

    let buf: &mut Vec<u8> = &mut vec![];
    io::stdin().lock().read_to_end(buf).unwrap();

    let split_lines: Vec<&str> = std::str::from_utf8(buf)
        .unwrap()
        .split('\n')
        .filter(|st| !st.is_empty())
        .collect();

    dbg!(&split_lines);

    let jboxes: Vec<Point> = split_lines
        .into_iter()
        .map(|t| {
            let [a, b, c]: [u32; 3] = t
                .split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            (a, b, c)
        })
        .collect();

    dbg!(&jboxes);

    let mut alr_paired: HashSet<HashSet<Point>> = HashSet::new();

    for _iter in 0..num_pairings {
        let p = find_closest_pairing(&jboxes, &alr_paired);
        alr_paired.insert(p);
        dbg!(&p);
    }
}

fn straight_line_dist(a: Point, b: Point) -> f32 {
    let dx = a.0.abs_diff(b.0) as f32;
    let dy = a.1.abs_diff(b.1) as f32;
    let dz = a.2.abs_diff(b.2) as f32;

    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn find_closest_pairing(v: &Vec<Point>, set: &HashSet<(Point, Point)>) -> (Point, Point) {
    // find current nearest pairing
    let mut dist_min: f32 = f32::MAX;
    let mut last_best = ((0, 0, 0), (0, 0, 0));
    for x in v {
        for y in v {
            let dist = straight_line_dist(*x, *y);
            if dist == 0.0 || set.contains(&(*x, *y)) || set.contains(&(*y, *x)) {
                continue;
            }
            if dist < dist_min {
                last_best = (*x, *y);
                dist_min = dist;
            }
        }
    }

    last_best
}
