#![feature(test)]

extern crate test;

use std::{cmp::Reverse, collections::BinaryHeap};

use anyhow::Result;
use rustc_hash::FxHashSet;

fn main() -> Result<()> {
    let stdin = std::io::read_to_string(std::io::stdin())?;

    println!("{}", solve(&stdin));
    Ok(())
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn solve(stdin: &str) -> usize {
    let width = stdin.chars().position(|ch| ch == '\n').unwrap();
    let height = (stdin.len() + 1) / (width + 1);
    let h_per = width - 2;
    let v_per = height - 2;

    let period = v_per * h_per / gcd(v_per, h_per);

    let mut blocked = vec![false; period * width * height];
    let to_index = |t, r, c| c + r * width + t * width * height;

    let normalise = |r: usize, c: usize| {
        let r_norm = (r - 1) % v_per + 1;
        let c_norm = (c - 1) % h_per + 1;
        (r_norm, c_norm)
    };

    for (r, line) in stdin.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            let (rd, cd) = match ch {
                'v' => (1, 0),
                '^' => (v_per - 1, 0),
                '<' => (0, h_per - 1),
                '>' => (0, 1),
                _ => continue,
            };

            for t in 0..period {
                let r1 = r + rd * t;
                let c1 = c + cd * t;
                let (r1, c1) = normalise(r1, c1);
                blocked[to_index(t, r1, c1)] = true;
            }
        }
    }

    let neighbours = |t: usize, r: usize, c: usize| {
        let blocked = &blocked;
        [(r - 1, c), (r, c - 1), (r + 1, c), (r, c + 1), (r, c)]
            .into_iter()
            .filter(|&(r, c)| r >= 1 && c >= 1 && r <= height - 2 && c <= width - 2)
            .filter(move |&(r, c)| !blocked[to_index(t, r, c)])
    };

    let fastest_from_to = |start: usize, from: (usize, usize), to: (usize, usize)| {
        let h = |(r, c): (usize, usize)| r.abs_diff(to.0) + c.abs_diff(to.1);

        let mut unvisited = BinaryHeap::new();
        let mut marked = FxHashSet::default();

        let min_dist = h(from);

        unvisited.push((Reverse(min_dist), start, from));

        while let Some((_, t, (r, c))) = unvisited.pop() {
            let t = t + 1;

            if (r, c) == to {
                return t;
            }

            for (r1, c1) in neighbours(t % 600, r, c) {
                if marked.insert((t, r1, c1)) {
                    unvisited.push((Reverse(t + h((r1, c1))), t, (r1, c1)));
                }
            }

            if (r, c) == from {
                unvisited.push((Reverse(t + min_dist), t, from));
            }
        }

        unreachable!()
    };

    let part1 = fastest_from_to(0, (0, 1), (height - 2, width - 2));
    let part2 = fastest_from_to(part1, (height - 1, width - 2), (1, 1));
    let part3 = fastest_from_to(part2, (0, 1), (height - 2, width - 2));

    part3
}

#[bench]
fn bench(bencher: &mut test::Bencher) {
    let input = include_str!("../../inputs/day24");

    bencher.iter(|| test::black_box(solve(input)));
}
