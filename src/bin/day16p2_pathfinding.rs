#![feature(once_cell)]
#![feature(saturating_int_impl)]
#![feature(test)]

extern crate test;

use std::{num::Saturating, collections::{VecDeque, BinaryHeap}, cmp::Reverse};

use aoc2022::lazily;
use regex::Regex;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = parse(&input);
    let answer = solve(&input);
    println!("{answer}");
}

fn not_brute_force(
    n_pressurised: usize,
    dist_c: &[Vec<usize>],
    pressures: &[usize],
) -> Vec<usize> {
    let mut max_released = vec![0; (1 << n_pressurised) * (n_pressurised + 1) * 27];
    let mut best_per_opened = vec![0; 1 << n_pressurised];
    let to_index = |time, position, opened| time + 27 * (position + (n_pressurised + 1) * opened);

    max_released[to_index(0, n_pressurised, 0)] = 0;
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), n_pressurised, 0));

    while let Some((Reverse(time), position, opened)) = queue.pop() {
        let released = max_released[to_index(time, position, opened)];
        best_per_opened[opened] = best_per_opened[opened].max(released);

        // println!("{time:>2} {position:>2} {opened:0>15b} {released:>4}");

        for neighbour in 0..n_pressurised {
            if opened & (1 << neighbour) == 0 {
                let new_time = time + dist_c[position][neighbour] + 1;

                if new_time <= 26 {
                    let a = &mut max_released[to_index(new_time,neighbour,opened | 1 << neighbour)];
                    if *a == 0 {
                        queue.push((Reverse(new_time), neighbour, opened | 1 << neighbour));
                    }
                    *a = (*a).max(released + (26 - new_time) * pressures[neighbour]);
                }
            }
        }
    }

    best_per_opened

    // (best, best_opened)
}

// calls `closure` on bitsets which are supersets of i, but subsets of i | extra
fn for_all_valid_subsets(i: usize, extra: usize, closure: &mut impl FnMut(usize)) {
    let one_pos = extra.trailing_zeros();
    if one_pos >= 15 {
        closure(i);
    } else {
        for_all_valid_subsets(i, extra & !(1 << one_pos), closure);
        for_all_valid_subsets(i | (1 << one_pos), extra & !(1 << one_pos), closure);
    }
}

fn solve(input: &Input) -> usize {
    let n = input.nodes.len();

    // Create a distance matrix for Floyd-Warshall
    let mut dist = vec![vec![Saturating(usize::MAX); n]; n];
    for (index, Node { neighbours, .. }) in input.nodes.iter().enumerate() {
        for &i in neighbours {
            dist[index][i] = Saturating(1);
        }
        dist[index][index] = Saturating(0);
    }

    // Perform Floyd-Warshall
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }

    // As a simplification, only look at the vaults which release pressure when opened
    let mut pressurised_valves = vec![];
    for (index, Node { pressure, .. }) in input.nodes.iter().enumerate() {
        if pressure > &0 {
            pressurised_valves.push(index);
        }
    }

    let n_pressurised = pressurised_valves.len();

    // "Compressed" distrance matrix
    let mut dist_c = vec![vec![usize::MAX; n_pressurised]; n_pressurised];

    for (i, real_i) in pressurised_valves.iter().enumerate() {
        for (j, real_j) in pressurised_valves.iter().enumerate() {
            dist_c[i][j] = dist[*real_i][*real_j].0;
        }
    }
    // include distances from root
    let last_column = pressurised_valves
        .iter()
        .map(|i| dist[input.root][*i].0)
        .collect();
    dist_c.push(last_column);

    // Mapping compressed index -> pressure
    let pressures: Vec<_> = pressurised_valves
        .iter()
        .map(|i| input.nodes[*i].pressure)
        .collect();

    let mut solutions = not_brute_force(n_pressurised, &dist_c, &pressures);

    // bitmask for negation
    let bitmask = (1 << n_pressurised) - 1;

    for i in 0..1 << n_pressurised {
        for_all_valid_subsets(i, !i & bitmask, &mut |j| {
            solutions[j] = solutions[j].max(solutions[i]);
        });
    }

    // find optimal subset choice
    (0..1 << (pressurised_valves.len() - 1))
        .map(|i| solutions[i] + solutions[!i & bitmask])
        .max()
        .unwrap()
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

    let mut name_to_index: FxHashMap<&str, usize> = <_>::default();
    let mut nodes_in_order = vec![];
    for (index, line) in input.lines().enumerate() {
        let pressure = digits.find(line).unwrap().as_str().parse().unwrap();

        let mut names = line.matches(regex);
        let from = names.next().unwrap();

        name_to_index.insert(from, index);
        nodes_in_order.push((names, pressure));
    }

    let mut nodes = vec![];
    for (neighbours, pressure) in nodes_in_order.into_iter() {
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

// #[bench]
fn bench(bencher: &mut test::Bencher) {
    let input = include_str!("../../inputs/day16");
    let input = parse(input);

    bencher.iter(|| test::black_box(solve(&input)));
}

#[bench]
fn bench_parse(bencher: &mut test::Bencher) {
    let input = include_str!("../../inputs/day16");

    bencher.iter(|| test::black_box(parse(input)));
}