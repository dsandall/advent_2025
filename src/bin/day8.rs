use core::str;
use std::collections::HashSet;
use std::io::{self, Read};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Point(u32, u32, u32);

impl Point {
    //pub fn equals() -> () {}
}

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
            Point(a, b, c)
        })
        .collect();

    dbg!(&jboxes);

    let mut alr_paired: Vec<HashSet<Point>> = Vec::new();

    for iter in 0..num_pairings {
        let p: HashSet<Point> = find_closest_pairing(&jboxes, &mut alr_paired);
        alr_paired.insert(0, p.clone());
        dbg!(iter, &alr_paired);
    }
}

fn straight_line_dist(a: Point, b: Point) -> f32 {
    let dx = a.0.abs_diff(b.0) as f32;
    let dy = a.1.abs_diff(b.1) as f32;
    let dz = a.2.abs_diff(b.2) as f32;

    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn find_closest_pairing(v: &Vec<Point>, set: &mut Vec<HashSet<Point>>) -> HashSet<Point> {
    // find current nearest pairing
    let mut dist_min: f32 = f32::MAX;
    let mut last_best = HashSet::new();

    let mut t = (*set).clone();

    for x in v {
        for y in v {
            let dist = straight_line_dist(x.clone(), y.clone());

            println!("{:?}{:?}{:?}", &x, &y, &dist);

            // if x is in one of the sets:
            //    and y is also in the same set:
            //    this pairing is invalid, continue

            if (t).iter().any(|s| s.contains(x) && s.contains(y)) {
                dbg!("CASE 1");
                break;
            }

            // if x and y are the same, invalid
            if dist == 0.0 {
                dbg!("CASE 2");
                continue;
            }
            if dist < dist_min {
                dbg!("CASE 3");

                for mut s in t {
                    if s.contains(x) || s.contains(y) {
                        if s.contains(x) {
                            s.insert(y.clone());
                        }
                    }
                }

                last_best.clear();
                last_best.insert(x.clone());
                last_best.insert(y.clone());
                dist_min = dist;
                continue;
            }

            dbg!("CASE 4");
        }
    }

    *set = t;

    last_best
}
