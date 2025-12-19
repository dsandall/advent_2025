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
            let s = num.to_string();
            let n = s.len();
            //dbg!(&s, n);
            if n % 2 == 0 {
                let s1 = &s[0..(n / 2)];
                let s2 = &s[n / 2..];
                //dbg!(s1);
                //dbg!(s2);
                if s1 == s2 {
                    dbg!(num);
                    count += num;
                }
            }
        }

        dbg!(count);
    }

    //
    //    // transform list of left rights to i32
    //    let nums_iter = handle.lines().filter_map(Result::ok).map(|l| {
    //        dbg!(&l);
    //
    //        let ranges = l.trim_ascii_end().split(',');
    //        dbg!(ranges);
    //    });
}
