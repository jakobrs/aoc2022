#![feature(test)]
#![feature(iter_array_chunks)]

extern crate test;

use std::{
    collections::{BTreeSet, HashSet},
    ops::{Index, IndexMut},
};

use regex::Regex;
use rustc_hash::FxHashSet;
use test::Bencher;

const WIDTH: usize = 1000;
const HEIGHT: usize = 200;

type Point = (usize, usize);

#[derive(Clone)]
struct Grid {
    // cells: [bool; HEIGHT * WIDTH],
    cells: FxHashSet<Point>,
}

impl Grid {
    fn contains(&self, point: &Point) -> bool {
        self.cells.contains(point)
    }

    fn insert(&mut self, point: Point) {
        self.cells.insert(point);
    }
}

fn drop(grid: &mut Grid, r: usize, c: usize) -> usize {
    let mut n = 0;

    if r == 199 {
        return n;
    }

    if !grid.contains(&(r + 1, c)) {
        n += drop(grid, r + 1, c);

        if !grid.contains(&(r + 1, c)) {
            return n;
        }
    }

    if !grid.contains(&(r + 1, c - 1)) {
        n += drop(grid, r + 1, c - 1);

        if !grid.contains(&(r + 1, c - 1)) {
            return n;
        }
    }

    if !grid.contains(&(r + 1, c + 1)) {
        n += drop(grid, r + 1, c + 1);

        if !grid.contains(&(r + 1, c + 1)) {
            return n;
        }
    }

    // only fill in cell if all three cells below it are also filled
    grid.insert((r, c));

    return n + 1;
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("{}", drop(&mut inner(&stdin), 0, 500));
}

fn inner(stdin: &str) -> Grid {
    let mut grid = Grid {
        cells: <_>::default(),
    };

    for line in stdin.lines() {
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
                    grid.insert((r, c));
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
