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
                    + (30 - new_t) * nodes[j].pressure as usize;
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

    let best = brute_force(0, input.root, &dist, 0, &pressurised_valves, &input.nodes);

    best
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

    bencher.iter(|| {
        test::black_box(solve(&input))
    });
}