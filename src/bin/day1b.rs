#![feature(pattern)]
use core::panic;
use std::io::{self, BufRead};
use std::iter;

const START: i32 = 50;

fn exclusive_scan(v: &[i32]) -> Vec<i32> {
    let mut sum = START;
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
    let mut count = 0;
    let mut prev = START;
    for s in scanned {
        dbg!("\n another one");
        let delta = s - prev;
        if delta == 0 {
            continue;
        }
        dbg!(delta);
        let to_right = delta > 0;

        // if spin was over 100:
        // sub 100 and add to count, then handle remainder
        if delta.abs() > 100 {
            count += delta.abs() / 100;
        }

        // if spin was right and not over 100: check if new modulo < old modulo (it looped over), then it clicked
        // if spin was left and not over 100: check if new modulo > old modulo, then it clicked

        let posfunc = |p: i32| {
            let mut r = p % 100;
            if r < 0 {
                r += 100;
            }
            r
        };

        let pp = posfunc(prev);
        let cp = posfunc(s);
        dbg!(pp, cp);

        if cp == 0 {
            count += 1;
        } else {
            if to_right & (cp < pp) {
                count += 1;
            }
            if !to_right & (cp > pp) & (pp != 0) {
                count += 1;
            }
        }

        dbg!(s, count);
        //if s % 100 == 0 {
        //    count += 1;
        //    dbg!("clicked");
        //};

        prev = s;
    }

    dbg!(count);
}
