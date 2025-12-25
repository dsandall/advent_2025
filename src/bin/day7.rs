#![feature(pattern)]
#![feature(breakpoint)]
use core::panic;
use std::{
    collections::{HashMap, VecDeque},
    io::{self, BufRead, Read},
};

struct Grid {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Grid {
    fn get(&self, p: (i32, i32)) -> Option<&u8> {
        if (0..self.width as i32).contains(&p.0) && (0..self.height as i32).contains(&p.1) {
            Some(&self.data[p.0 as usize + p.1 as usize * self.width])
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

    let mut count: u64 = 0;
    type Saved = u32;
    let mut prev: HashMap<(usize, usize), Saved> = HashMap::new();

    for xterm in 0..g.width {
        // initial possible ending point
        let mut beamfronts: VecDeque<(i32, i32)> = VecDeque::new();
        beamfronts.push_back((xterm as i32, (g.height - 1) as i32));

        dbg!("new start point");
        dbg!(beamfronts.iter().peekable().peek());

        // parse all possible paths to the end point
        while beamfronts.len() != 0 {
            let (mut x, mut y): (i32, i32) = beamfronts.pop_front().unwrap();

            let paths = 1;
            loop {
                // check if this could be the start of a split beam
                let (l, r) = ((x + 1, y), (x - 1, y));
                for n in [l, r] {
                    if g.get(n).is_some_and(|c| *c == b'^') {
                        // record split
                        beamfronts.push_back(n);
                    }
                }

                // trace back regular beams, or terminate it
                let up = (x, y - 1);
                match g.get(up) {
                    Some(b'S') => {
                        // positive terminating condition, save to count
                        //TODO:
                        count += paths;
                        if count.is_multiple_of(10000) {
                            dbg!(count);
                        }
                        break;
                    }
                    Some(b'.') => {
                        // trace propagation further back
                        y -= 1;
                    }
                    Some(b'^') | None => {
                        // end of path, disregard current count
                        // no non-split beams can come from directly below a splitter
                        break;
                    }
                    _ => {
                        panic!();
                    }
                }
            }
        }
    }

    dbg!(count);
}
