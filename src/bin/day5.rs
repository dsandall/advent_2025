use std::{
    io::{self, BufRead},
    ops::Range,
};

fn main() {
    let handle = io::stdin().lock();

    let split: (Vec<String>, Vec<String>) = handle
        .lines()
        .map(|a| a.unwrap())
        .partition(|l| l.contains('-'));

    dbg!(&split);

    let res = split.1[1..].iter().filter_map(|a| {
        let i: u64 = a.parse().unwrap();
        dbg!(i);

        let r = split.0.iter().find(|range| {
            let (ls, us) = range.split_once('-').unwrap();
            let range: Range<u64> = ls.parse().unwrap()..(us.parse::<u64>().unwrap() + 1);
            dbg!(&range);
            range.contains(&i)
        });
        dbg!(r);
        r
    });

    let v: Vec<&String> = res.collect();

    dbg!(&v);
    dbg!(&v.len());
}
