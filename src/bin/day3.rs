#![feature(pattern)]
use std::{
    io::{self, BufRead},
    num,
    ops::Range,
};

fn main() {
    let handle = io::stdin().lock();
    const NUM_BATS: usize = 12;

    //calculate optimal pack voltage for each line
    let nums_iter = handle.lines().filter_map(Result::ok).map(|l| {
        dbg!(&l);
        let lb = l.as_bytes();

        // define picking function given some range in the string
        let search = |range: Range<usize>| {
            dbg!(&l[range.clone()]);

            let switch_bigger = |cur: Option<(usize, u8)>, next: (usize, u8)| {
                match cur {
                    // select the max num, but do not switch in cases of equality
                    // conservative search
                    None => Some(next),
                    Some(best) => {
                        if next.1 > best.1 {
                            Some(next)
                        } else {
                            Some(best)
                        }
                    }
                }
            };

            lb[range]
                .iter()
                .map(|a| a - 48)
                .enumerate()
                .fold(None, switch_bigger)
                .unwrap()
        };

        // pick each subsequent voltage, define ranges that leave room for future sequential
        // voltage picks
        let picks: Vec<u8> = (0..NUM_BATS)
            .rev()
            .fold(
                (0, Vec::new()), // (start_idx, remaining, results)
                |(start, mut acc), nr| {
                    let range = start..(l.len() - nr);
                    dbg!(&range);

                    let (ind, val) = search(range);
                    dbg!(ind, val);

                    acc.push(val);
                    (ind + start + 1, acc)
                },
            )
            .1;

        // collect and concatenate voltage picks
        picks.iter().rev().enumerate().fold(0, |acc, el| -> u64 {
            acc + 10_u64.pow(el.0 as u32) * (*el.1 as u64)
        })
    });

    // sum all concatenated pack voltages
    let a: u64 = nums_iter.sum();
    dbg!(a);
}
