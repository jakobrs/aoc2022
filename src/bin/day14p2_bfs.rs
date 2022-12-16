#![feature(test)]
#![feature(iter_array_chunks)]

extern crate test;

use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    ops::{Index, IndexMut},
};

use bitvec::prelude::*;
use regex::Regex;
use rustc_hash::FxHashSet;
use test::Bencher;

const WIDTH: usize = 1000;
const HEIGHT: usize = 200;

type Point = (usize, isize);

#[derive(Clone)]
struct Grid {
    cells: BitVec<u64, Lsb0>,
    // cells: FxHashSet<Point>,
    r_max: usize,
}

impl Grid {
    fn to_index((r, c): Point) -> usize {
        r * WIDTH * 2 + (c + WIDTH as isize) as usize
    }

    fn contains(&self, point: &Point) -> bool {
        self.cells[Self::to_index(*point)]
    }

    fn insert(&mut self, point: Point) {
        self.cells.set(Self::to_index(point), true);
    }
}

fn drop(grid: &mut Grid, point: Point) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(point);
    let mut n = 0;

    while let Some((r, c)) = queue.pop_front() {
        n += 1;

        if r <= grid.r_max {
            for offset in -1..=1 {
                let new_point = (r + 1, c + offset);
                if !grid.contains(&new_point) {
                    grid.insert(new_point);
                    queue.push_back(new_point);
                }
            }
        }
    }

    return n;
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("{}", drop(&mut inner(&stdin), (0, 500)));
}

fn inner(stdin: &str) -> Grid {
    let mut grid = Grid {
        cells: BitVec::repeat(false, HEIGHT * WIDTH * 2),
        r_max: 0,
    };

    for line in stdin.lines() {
        let mut pairs = line
            .split(&['-', '>', ' ', ','])
            .filter(|i| !i.is_empty())
            .array_chunks()
            .map(|[m1, m2]| (m1.parse::<isize>().unwrap(), m2.parse::<usize>().unwrap()));

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
            grid.r_max = grid.r_max.max(r_max);
        }
    }

    grid
}

// drop(&mut grid, 0, 500)

#[bench]
fn parse(bencher: &mut Bencher) {
    let stdin = std::include_str!("../../inputs/day14");

    bencher.iter(|| test::black_box(inner(&stdin)))
}

#[bench]
fn both(bencher: &mut Bencher) {
    let stdin = std::include_str!("../../inputs/day14");

    bencher.iter(|| test::black_box(drop(&mut inner(&stdin), (0, 500))))
}

#[bench]
fn part2(bencher: &mut Bencher) {
    let stdin = std::include_str!("../../inputs/day14");
    let parsed = inner(&stdin);

    bencher.iter(|| test::black_box(drop(&mut parsed.clone(), (0, 500))))
}
