#![feature(pattern)]
use core::panic;
use std::io::{self, Read};

fn main() {
    let buf = &mut vec![];
    io::stdin().lock().read_to_end(buf).unwrap();

    let split_lines: Vec<&[u8]> = buf.splitn(100, |s| *s == b'\n').collect();

    let (rows, cols) = (split_lines.len() - 1, split_lines[0].len());
    dbg!(rows, cols);
    dbg!(&split_lines);

    let mut it = split_lines.iter().rev().skip(1).rev();
    //let s: &str = it.next_back().unwrap();
    //dbg!(s, it);

    let split_cols: Vec<String> = (0..cols)
        .map(|c| {
            (0..rows)
                .map(|r| {
                    dbg!(r, c);
                    split_lines[r][c] as char
                })
                .collect()
        })
        .collect::<Vec<String>>()
        .into_iter()
        .filter(|s| !s.trim().is_empty())
        .collect();

    dbg!(&split_cols);

    let res = split_cols
        .iter()
        .fold((0, 0, 0), |acc: (u64, u64, u8), v: &String| {
            let num;
            let mut ncc = acc;
            if v.contains([b'+' as char, b'*' as char]) {
                dbg!("changing ops");
                // parse the sign
                ncc.2 = *v.as_bytes().last().unwrap();

                // accumulate last
                ncc.0 += ncc.1;

                // reset mini accumulator
                if v.contains([b'+' as char]) {
                    ncc.1 = 0;
                } else {
                    ncc.1 = 1;
                }

                // parse number without the sign
                let mut chars = v.chars();
                chars.next_back();
                num = chars.as_str().trim().parse::<u64>().unwrap();
            } else {
                // parse number
                num = v.trim().parse::<u64>().unwrap();
            }
            dbg!(v);
            dbg!(num);

            // use mini accum
            match ncc.2 {
                b'*' => ncc.1 *= num,
                b'+' => ncc.1 += num,
                _ => panic!(),
            };

            dbg!(ncc);
            ncc
        });
    dbg!(res);

    // add final result to big accum
    let finres = res.0 + res.1;
    dbg!(finres);
}
