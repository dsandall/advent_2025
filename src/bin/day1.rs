#![feature(pattern)]
use core::panic;
use std::io::{self, BufRead};
use std::iter;

fn exclusive_scan(v: &[i32]) -> Vec<i32> {
    let mut sum = 50;
    v.iter()
        .map(|&x| {
            let old = sum;
            sum += x;
            old
        })
        .collect()
}

fn main() {
    let handle = io::stdin().lock();

    // transform list of left rights to i32
    let nums_iter = handle.lines().filter_map(Result::ok).map(|l| {
        dbg!(&l);

        let n: i32 = l
            .trim_start_matches(|c: char| c.is_alphabetic())
            .parse()
            .unwrap();
        dbg!(n);

        match l.as_bytes()[0] {
            b'R' => n,
            b'L' => -n,
            _ => {
                panic!();
            }
        }
    });

    let v: Vec<i32> = nums_iter.collect();
    for ln in &v {
        dbg!(ln);
    }

    // lazy blelloch scan, could be parallelized easily
    let scanned = exclusive_scan(&v);

    // count times where you stopped at 0
    let mut count: i32 = 0;
    for s in scanned {
        dbg!(s);
        dbg!(s % 100);
        if (s % 100 == 0) {
            count += 1
        };
    }

    dbg!(count);
}
