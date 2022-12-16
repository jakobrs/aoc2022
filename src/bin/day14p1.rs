#![feature(test)]
#![feature(iter_array_chunks)]

extern crate test;

use std::ops::{Index, IndexMut};

use bitvec::prelude::*;
use regex::Regex;
use test::Bencher;

const WIDTH: usize = 1000;
const HEIGHT: usize = 200;

#[derive(Clone)]
struct Grid {
    // cells: Box<[bool; HEIGHT * WIDTH]>,
    cells: Box<BitArray<[u8; WIDTH * HEIGHT / 8], Lsb0>>,
}

type Point = (usize, usize);

impl Grid {
    fn to_index((r, c): Point) -> usize {
        r * WIDTH + c
    }

    fn set(&mut self, point: Point, value: bool) {
        self.cells.set(Self::to_index(point), value);
    }
}

impl Index<Point> for Grid {
    type Output = bool;

    fn index(&self, point: Point) -> &Self::Output {
        &self.cells[Self::to_index(point)]
    }
}

// impl IndexMut<(usize, usize)> for Grid {
//     fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut Self::Output {
//         &mut self.cells[r * WIDTH + c]
//     }
// }

fn drop(grid: &mut Grid, r: usize, c: usize) -> usize {
    let mut n = 0;

    if r == 199 {
        return n;
    }

    if !grid[(r + 1, c)] {
        n += drop(grid, r + 1, c);

        if !grid[(r + 1, c)] {
            return n;
        }
    }

    if !grid[(r + 1, c - 1)] {
        n += drop(grid, r + 1, c - 1);

        if !grid[(r + 1, c - 1)] {
            return n;
        }
    }

    if !grid[(r + 1, c + 1)] {
        n += drop(grid, r + 1, c + 1);

        if !grid[(r + 1, c + 1)] {
            return n;
        }
    }

    // only fill in cell if all three cells below it are also filled
    grid.set((r, c), true);

    return n + 1;
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("{}", drop(&mut inner(&stdin), 0, 500));
}

fn inner(stdin: &str) -> Grid {
    let mut grid = Grid {
        // cells: Box::new([false; HEIGHT * WIDTH]),
        cells: Box::new(BitArray::new([0u8; WIDTH * HEIGHT / 8])),
    };

    let mut lines: Vec<&str> = stdin.lines().collect();
    // lines.sort();
    // lines.dedup();

    for line in lines {
        let mut pairs = line
            .split(&['-', '>', ' ', ','])
            .filter(|i| !i.is_empty())
            .array_chunks()
            .map(|[m1, m2]| (m1.parse::<usize>().unwrap(), m2.parse::<usize>().unwrap()));

        // let mut pairs = regex.find_iter(line).array_chunks().map(|[m1, m2]| {
        //     (
        //         m1.as_str().parse::<usize>().unwrap(),
        //         m2.as_str().parse::<usize>().unwrap(),
        //     )
        // });

        let Some(mut last) = pairs.next() else { continue; };

        for next in pairs {
            // draw line from last to next
            let r_min = last.1.min(next.1);
            let r_max = last.1.max(next.1);
            let c_min = last.0.min(next.0);
            let c_max = last.0.max(next.0);
            for r in r_min..=r_max {
                for c in c_min..=c_max {
                    // grid[(r, c)] = true;
                    grid.set((r, c), true);
                }
            }

            last = next;
        }
    }

    grid
}

// drop(&mut grid, 0, 500)

#[bench]
fn bench(bencher: &mut Bencher) {
    let stdin = std::include_str!("../../inputs/day14");

    bencher.iter(|| test::black_box(drop(&mut inner(&stdin), 0, 500)))
}

#[bench]
fn bench_1(bencher: &mut Bencher) {
    let stdin = std::include_str!("../../inputs/day14");
    let parsed = inner(&stdin);

    bencher.iter(|| test::black_box(drop(&mut parsed.clone(), 0, 500)))
}
