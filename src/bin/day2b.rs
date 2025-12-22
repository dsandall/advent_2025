#![feature(pattern)]
use std::io::{self, BufRead};

fn main() {
    let mut handle = io::stdin().lock();
    let mut buf = String::new();
    handle.read_line(&mut buf).unwrap();

    let newb = buf.trim_ascii_end().split(',');
    let mut count = 0;

    for p in newb {
        let (start, end) = p.split_once('-').unwrap();
        dbg!(start, end);
        let range: std::ops::Range<u64> = start.parse().unwrap()..(end.parse::<u64>().unwrap() + 1);
        for num in range {
            if check_n(num, 2)
                | check_n(num, 3)
                | check_n(num, 4)
                | check_n(num, 5)
                | check_n(num, 6)
                | check_n(num, 7)
                | check_n(num, 8)
                | check_n(num, 9)
                | check_n(num, 10)
            {
                count += num;
                dbg!(num);
            };
        }
        dbg!(count);
    }
}

fn check_n(num: u64, n: usize) -> bool {
    let s = num.to_string();
    let l = s.len();
    if l < n {
        return false;
    }
    //dbg!(&s, n);

    if !l.is_multiple_of(n) {
        return false;
    }

    let chunkl = l / n;
    let first = &s.as_bytes()[..chunkl];

    s.as_bytes().chunks(chunkl).all(|c| c == first)
}
