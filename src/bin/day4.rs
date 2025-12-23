#![feature(pattern)]
use core::panic;
use std::io::{self, BufRead, Read};

struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<(T, bool)>,
}

impl<T> Grid<T> {
    fn get(&self, x: usize, y: usize) -> &bool {
        &self.data[x + y * self.width].1
    }
    fn get_mut(&mut self, x: usize, y: usize) -> &mut bool {
        &mut self.data[x + y * self.width].1
    }
}

fn main() {
    let mut handle = io::stdin().lock();

    let mut g: Grid<u8> = Grid {
        width: 0,
        height: 0,
        data: Vec::new(),
    };
    let mut bufg = Vec::new();
    handle.read_to_end(&mut bufg).unwrap();

    g.width = bufg.iter().enumerate().find(|c| *c.1 == b'\n').unwrap().0;

    g.data = bufg
        .iter()
        .filter_map(|c| match *c {
            b'\n' => None,
            b'.' => Some((*c, false)),
            b'@' => Some((*c, true)),
            _ => None,
        })
        .collect();

    g.height = g.data.len() / g.width;
    dbg!(g.width, g.height, g.data.len());
    let mut count = 0;

    loop {
        let is_roll = |x, y| *g.get(x, y);
        let in_bounds =
            |x: i32, y: i32| x >= 0 && y >= 0 && (x as usize) < g.width && (y as usize) < g.height;

        let forkliftables: Vec<(usize, usize)> = {
            (0..g.height)
                .flat_map(|y| (0..g.width).map(move |x| (x, y)))
                .filter_map(|(x, y)| {
                    // for each grid spot,
                    if !is_roll(x, y) {
                        None
                    } else {
                        // for each spot near the paper roll,
                        let surrounding_count = (y as i32 - 1..y as i32 + 2)
                            .flat_map(|dy| (x as i32 - 1..x as i32 + 2).map(move |dx| (dx, dy)))
                            .filter(|&(dx, dy)| {
                                in_bounds(dx, dy) && is_roll(dx as usize, dy as usize)
                            })
                            .count();

                        // counts itself, so <5
                        if surrounding_count < 4 + 1 {
                            Some((x, y))
                        } else {
                            None
                        }
                    }
                })
        }
        .collect();

        let found = &forkliftables.len();
        dbg!(found);

        if *found == 0 {
            break;
        }

        count += found;

        for (x, y) in forkliftables {
            *g.get_mut(x, y) = false;
        }
    }

    dbg!(count);
}
