#![feature(once_cell)]
#![feature(saturating_int_impl)]
#![feature(test)]

extern crate test;

use std::num::Saturating;

use aoc2022::lazily;
use regex::Regex;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = parse(&input);
    let a = solve(&input);
    println!("{a}");
}

fn brute_force(
    t: usize,
    i: usize,
    dist: &[Vec<Saturating<usize>>],
    seen: usize,
    pressurised_valves: &[usize],
    nodes: &[Node],
) -> usize {
    let mut best = 0;

    for &j in pressurised_valves {
        if (seen & (1 << j)) == 0 {
            let new_t = t + dist[i][j].0 + 1;
            if new_t <= 30 {
                let a = brute_force(new_t, j, dist, seen | (1 << j), pressurised_valves, nodes)
                    + (30 - new_t) * nodes[j].pressure;
                if a > best {
                    best = a;
                }
            }
        }
    }

    best
}

fn solve(input: &Input) -> usize {
    let n = input.nodes.len();

    let mut dist = vec![vec![Saturating(usize::MAX); n]; n];
    for (index, Node { neighbours, .. }) in input.nodes.iter().enumerate() {
        for &i in neighbours {
            dist[index][i] = Saturating(1);
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }

    let mut pressurised_valves = vec![];
    for (index, Node { pressure, .. }) in input.nodes.iter().enumerate() {
        if pressure > &0 {
            pressurised_valves.push(index);
        }
    }

    brute_force(0, input.root, &dist, 0, &pressurised_valves, &input.nodes)
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Node {
    neighbours: SmallVec<[usize; 2]>,
    pressure: usize,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    nodes: Vec<Node>,
    root: usize,
}

fn parse(input: &str) -> Input {
    let regex = lazily!(Regex::new(r"[A-Z]{2}").unwrap());
    let digits = lazily!(Regex::new(r"\d+").unwrap());

    let mut name_to_index: FxHashMap<&str, usize> = FxHashMap::default();
    let mut nodes_in_order = vec![];
    for (index, line) in input.lines().enumerate() {
        let pressure = digits.find(line).unwrap().as_str().parse().unwrap();

        let mut names = line.matches(regex);
        let from = names.next().unwrap();

        name_to_index.insert(from, index);
        nodes_in_order.push((names, pressure));
    }

    let mut nodes = vec![];
    for (neighbours, pressure) in nodes_in_order {
        nodes.push(Node {
            neighbours: neighbours.map(|node| name_to_index[node]).collect(),
            pressure,
        });
    }

    Input {
        nodes,
        root: name_to_index["AA"],
    }
}

#[bench]
fn bench(bencher: &mut test::Bencher) {
    let input = include_str!("../../inputs/day16");
    let input = parse(input);

    bencher.iter(|| {
        test::black_box(solve(&input))
    });
}