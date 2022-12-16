#![feature(test)]

extern crate test;

use std::ops::{Index, IndexMut};

use regex::Regex;
use test::Bencher;

struct Grid {
    cells: [bool; 200 * 1000],
}

impl Index<(usize, usize)> for Grid {
    type Output = bool;

    fn index(&self, (r, c): (usize, usize)) -> &Self::Output {
        &self.cells[r * 1000 + c]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut Self::Output {
        &mut self.cells[r * 1000 + c]
    }
}

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
    grid[(r, c)] = true;

    return n + 1;
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("{}", drop(&mut inner(&stdin), 0, 500));
}

fn inner(stdin: &str) -> Grid {
    let regex = Regex::new(r"(\d+),(\d+)").unwrap();
    let mut grid = Grid {
        cells: [false; 200 * 1000],
    };

    for line in stdin.lines() {
        let mut pairs = regex.captures_iter(line).map(|m| {
            (
                m.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                m.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            )
        });

        let Some(mut last) = pairs.next() else { continue; };

        for next in pairs {
            // draw line from last to next
            let r_min = last.1.min(next.1);
            let r_max = last.1.max(next.1);
            let c_min = last.0.min(next.0);
            let c_max = last.0.max(next.0);
            for r in r_min..=r_max {
                for c in c_min..=c_max {
                    grid[(r, c)] = true;
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
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();

    bencher.iter(|| test::black_box(inner(&stdin)))
}
