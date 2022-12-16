#![feature(once_cell)]
#![feature(saturating_int_impl)]
#![feature(test)]

extern crate test;

use std::num::Saturating;

use regex::Regex;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

// macro_rules! lazily {
//     ($ty:ty, $expr:expr) => {{
//         static LOCK: ::std::sync::LazyLock<$ty> = ::std::sync::LazyLock::new(|| $expr);

//         &*LOCK
//     }};
// }

macro_rules! lazily {
    ($expr:expr) => {{
        static LAZY: ::std::sync::LazyLock<
            ::std::boxed::Box<
                dyn ::std::any::Any + ::std::marker::Sync + ::std::marker::Send + 'static,
            >,
        > = ::std::sync::LazyLock::new(|| Box::new($expr));

        fn infer_type<T>(_: fn() -> T) -> &'static T {
            LAZY.downcast_ref().unwrap()
        }
        infer_type(|| $expr)
    }};
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = parse(&input);
    let answer = solve(&input);
    println!("{answer}");
}

// t is the amount of time used, i is the node the player is currently located at,
// dist[i][j] is the distance from vault i to j, seen is a bitset of all the vents that have been
// seen, pressures is a mapping from vault index to the pressure released by that vault.
fn brute_force(t: usize, i: usize, dist: &[Vec<usize>], seen: usize, pressures: &[usize]) -> usize {
    let mut best = 0;

    for j in 0..dist.len() - 1 {
        if (seen & (1 << j)) == 0 {
            let new_t = t + dist[i][j] + 1;
            if new_t <= 26 {
                let a = brute_force(new_t, j, dist, seen | (1 << j), pressures)
                    + (26 - new_t) * pressures[j];
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

    // "Compressed" distrance matrix
    let mut dist_c = vec![vec![usize::MAX; pressurised_valves.len()]; pressurised_valves.len()];

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
        .map(|i| input.nodes[*i].pressure as usize)
        .collect();

    // try half of all subsets
    let solutions: Vec<usize> = (0..1 << pressurised_valves.len() - 1)
        .map(|denied| brute_force(0, pressurised_valves.len(), &dist_c, denied, &pressures))
        .collect();

    // bitmask for negation
    let bitmask = (1 << (pressurised_valves.len() - 1)) - 1;
    // find optimal subset choice
    (0..1 << (pressurised_valves.len() - 1))
        .map(|i| solutions[i] + solutions[!i & bitmask])
        .max()
        .unwrap()
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Node {
    neighbours: SmallVec<[usize; 2]>,
    pressure: u16,
    index: u16,
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
    for (index, (neighbours, pressure)) in nodes_in_order.into_iter().enumerate() {
        nodes.push(Node {
            neighbours: neighbours.map(|node| name_to_index[node]).collect(),
            pressure: pressure,
            index: index as u16,
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
