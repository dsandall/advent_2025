use std::{
    io::{self, BufRead},
    ops::{Range, RangeBounds},
};

fn merge_ranges(mut ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
    if ranges.is_empty() {
        return ranges;
    }

    // 1. Sort by start, then end
    ranges.sort_by(|a, b| a.start.cmp(&b.start).then(a.end.cmp(&b.end)));

    let mut merged = Vec::new();
    let mut current = ranges[0].clone();

    for r in ranges.into_iter().skip(1) {
        if r.start <= current.end {
            // Overlapping or touching: merge
            current.end = current.end.max(r.end);
        } else {
            // Disjoint: commit current and start new
            merged.push(current);
            current = r;
        }
    }

    merged.push(current);
    merged
}

fn main() {
    let handle = io::stdin().lock();

    let (ranges, items): (Vec<String>, Vec<String>) = handle
        .lines()
        .map(|a| a.unwrap())
        .partition(|l| l.contains('-'));

    let ranges: Vec<Range<u64>> = ranges
        .iter()
        .map(|range| {
            let (ls, us) = range.split_once('-').unwrap();
            ls.parse().unwrap()..(us.parse::<u64>().unwrap() + 1)
        })
        .collect();

    let items: Vec<u64> = items.iter().skip(1).map(|a| a.parse().unwrap()).collect();

    dbg!(&ranges.len());
    dbg!(&ranges);
    dbg!(&items.len());
    dbg!(&items);

    let res = items
        .iter()
        .filter(|&i| {
            dbg!(i);
            ranges.iter().any(|range| range.contains(i))
        })
        .count();
    dbg!(res);

    let res2 = merge_ranges(ranges)
        .iter()
        .fold(0, |acc: u64, range| acc + range.end - range.start);
    dbg!(res2);
}
