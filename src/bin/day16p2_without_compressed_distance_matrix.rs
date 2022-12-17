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
    let answer = solve(&input);
    println!("{answer}");
}

// t is the amount of time used, i is the node the player is currently located at,
// pressurised_valves is a list of all the valves that release pressure when opened,
// dist[i][j] is the distance from vault i to j, seen is a bitset of all the vents that have been
// seen, and nodes is the list of nodes (used for reading the pressure)
fn brute_force(
    pressurised_valves: &[usize],
    dist: &[Vec<Saturating<usize>>],
    nodes: &[Node],
    t: usize,
    i: usize,
    seen: usize,
) -> (usize, usize) {
    let mut best = 0;
    let mut best_used = 0;

    for (seen_j, &j) in pressurised_valves.iter().enumerate() {
        if (seen & (1 << seen_j)) == 0 {
            let new_t = t + dist[i][j].0 + 1;
            if new_t <= 26 {
                let (mut a, mut used) = brute_force(
                    pressurised_valves,
                    dist,
                    nodes,
                    new_t,
                    j,
                    seen | (1 << seen_j),
                );
                a += (26 - new_t) * nodes[j].pressure;
                used |= 1 << seen_j;

                if a > best {
                    best = a;
                    best_used = used;
                }
            }
        }
    }

    (best, best_used)
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

    let mut pressurised_valves = vec![];
    for (index, Node { pressure, .. }) in input.nodes.iter().enumerate() {
        if pressure > &0 {
            pressurised_valves.push(index);
        }
    }

    let n_pressurised = pressurised_valves.len();

    // bitmask for negation
    let bitmask = (1 << n_pressurised) - 1;

    // Find optimal part 1 solution for each subset
    let mut solutions = vec![usize::MAX; 1 << n_pressurised];

    for i in 0..1 << n_pressurised {
        if solutions[i] == usize::MAX {
            let (optimum, used) =
                brute_force(&pressurised_valves, &dist, &input.nodes, 0, input.root, i);
            // extra represents the set of bits whose addition to `i` wouldn't change the outcome
            let extra = bitmask & !i & !used;
            for_all_valid_subsets(i, extra, &mut |j| {
                solutions[j] = optimum;
            });
        }
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

#[bench]
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
