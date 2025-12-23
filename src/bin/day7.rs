#![feature(pattern)]
use core::panic;
use std::io::{self, BufRead, Read};

struct Grid {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Grid {
    fn get(&self, p: (usize, usize)) -> Option<&u8> {
        if p.0 < self.width && p.1 < self.height {
            Some(&self.data[p.0 + p.1 * self.width])
        } else {
            None
        }
    }
    fn get_mut(&mut self, p: (usize, usize)) -> Option<&mut u8> {
        if p.0 < self.width && p.1 < self.height {
            Some(&mut self.data[p.0 + p.1 * self.width])
        } else {
            None
        }
    }
    fn disp(&self) {
        let v = self.data.chunks(self.width);
        let v = v.map(|a| a.iter().map(|a| *a as char).collect());
        dbg!(v.collect::<Vec<String>>());
    }
}

fn main() {
    let mut g: Grid = Grid {
        width: 0,
        height: 0,
        data: Vec::new(),
    };

    let mut bufg = Vec::new();
    let mut handle = io::stdin().lock();
    handle.read_to_end(&mut bufg).unwrap();

    g.width = bufg.iter().enumerate().find(|c| *c.1 == b'\n').unwrap().0;
    g.data = bufg.into_iter().filter(|&c| c != b'\n').collect();
    g.height = g.data.len() / g.width;

    dbg!(g.width, g.height);
    g.disp();

    let mut beamfronts: Vec<(usize, usize)> = (0..g.width)
        .filter_map(|x| {
            if *g.get((x, 0)).unwrap() == b'S' {
                Some((x, 0))
            } else {
                None
            }
        })
        .collect();

    let mut count = 0;
    for _ in 0..g.height - 1 {
        let mut new_fronts = Vec::new();

        for (x, y) in beamfronts.iter() {
            let p = (*x, *y + 1);

            match g.get(p) {
                Some(b'^') => {
                    let r = (*x + 1, *y + 1);
                    let l = (*x - 1, *y + 1);
                    for n in [r, l] {
                        if g.get_mut(n).is_some() {
                            *g.get_mut(n).unwrap() = b'|';
                            new_fronts.push(n);
                        } else {
                            panic!();
                        }
                    }

                    // count splits
                    count += 1;
                }
                Some(b'.') => {
                    *g.get_mut(p).unwrap() = b'|';
                    new_fronts.push(p);
                }
                Some(b'|') => {}
                _ => {
                    dbg!(*g.get(p).unwrap() as char);
                    panic!();
                }
            }
        }

        // Append new fronts after iteration
        beamfronts = new_fronts;

        dbg!(&beamfronts);
        g.disp();
    }

    dbg!(count);
}
