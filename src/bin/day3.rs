#![feature(pattern)]
use std::io::{self, BufRead};

fn main() {
    let handle = io::stdin().lock();

    let nums_iter = handle.lines().filter_map(Result::ok).map(|l| {
        dbg!(&l);

        let num_left = 1;
        dbg!(&l[..l.len() - num_left]);
        let max_elem = &l.as_bytes()[..l.len() - 1]
            .iter()
            .enumerate()
            .fold(None, |cur, next| match cur {
                None => Some(next),
                Some(best) => {
                    if next.1 > best.1 {
                        Some(next)
                    } else {
                        Some(best)
                    }
                }
            })
            .unwrap();
        dbg!(&max_elem);
        dbg!(&l[(max_elem.0 + 1)..]);

        let second_elem = &l.as_bytes()[(max_elem.0 + 1)..]
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.cmp(b.1))
            .unwrap();
        dbg!(&second_elem);

        let r = ((max_elem.1 - 48) * 10 + (second_elem.1 - 48)) as u32;
        dbg!(&r);
        r
    });

    let a: u32 = nums_iter.sum();
    dbg!(a);
}
