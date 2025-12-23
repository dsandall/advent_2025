#![feature(pattern)]
use std::io::{self, Read};

fn main() {
    let buf = &mut vec![];
    io::stdin().lock().read_to_end(buf).unwrap();

    let split_lines: Vec<Vec<&str>> = buf
        .splitn(100, |s| *s == b'\n')
        .map(|l| str::from_utf8(l).unwrap().split_whitespace().collect())
        .collect();

    let (rows, cols) = (split_lines.len() - 1, split_lines[0].len());
    dbg!(rows, cols);

    let split_cols: Vec<Vec<&str>> = (0..cols)
        .map(|c| {
            (0..rows)
                .map(|r| {
                    dbg!(r, c);
                    split_lines[r][c]
                })
                .collect()
        })
        .collect();

    dbg!(&split_lines, &split_cols);

    let res = split_cols.iter().fold(0, |acc, v| {
        let mut it = v.iter();
        let res: u64 = match it.next_back().unwrap().as_bytes()[0] {
            b'*' => it.fold(1, |inner, &n| inner * n.parse::<u64>().unwrap()),
            _ => it.fold(0, |inner, &n| inner + n.parse::<u64>().unwrap()),
        };
        acc + res
    });
    dbg!(res);
}
