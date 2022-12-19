#![feature(array_try_map)]

use std::collections::VecDeque;

use aoc2022::lazily;
use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let blueprints = parse(&input);

    let n: usize = blueprints
        .iter()
        .take(3)
        .map(|blueprint| simulate(blueprint) as usize)
        .product();
    println!("{n}");
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,

    ore_bots: u16,
    clay_bots: u16,
    obsidian_bots: u16,
    geode_bots: u16,
}

impl Default for State {
    fn default() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,

            ore_bots: 1,
            clay_bots: 0,
            obsidian_bots: 0,
            geode_bots: 0,
        }
    }
}

fn simulate(blueprint: &Blueprint) -> u16 {
    let mut frontier = Vec::new();
    let mut next = Vec::new();
    let mut parent = FxHashSet::default();

    frontier.push(State::default());

    for time in 0..32 {
        let size = frontier.len();
        println!("{time} {size}");
        for (i, state) in frontier.drain(..).enumerate() {
            if i % 1_000_000 == 0 {
                println!("{i}/{size}");
            }

            let mut new_state = state.clone();
            new_state.ore += state.ore_bots;
            new_state.clay += state.clay_bots;
            new_state.obsidian += state.obsidian_bots;
            new_state.geode += state.geode_bots;

            if !parent.contains(&new_state) {
                parent.insert(new_state);
                next.push(new_state);
            }

            if state.ore_bots < 4 && state.ore >= blueprint.ore_cost {
                let mut new_state = new_state.clone();
                new_state.ore -= blueprint.ore_cost;
                new_state.ore_bots += 1;
                if !parent.contains(&new_state) {
                    parent.insert(new_state);
                    next.push(new_state);
                }
            }
            if state.clay_bots < blueprint.obsidian_cost.1 && state.ore >= blueprint.clay_cost {
                let mut new_state = new_state.clone();
                new_state.ore -= blueprint.clay_cost;
                new_state.clay_bots += 1;
                if !parent.contains(&new_state) {
                    parent.insert(new_state);
                    next.push(new_state);
                }
            }
            if state.obsidian_bots < blueprint.geode_cost.1
                && state.ore >= blueprint.obsidian_cost.0
                && state.clay >= blueprint.obsidian_cost.1
            {
                let mut new_state = new_state.clone();
                new_state.ore -= blueprint.obsidian_cost.0;
                new_state.clay -= blueprint.obsidian_cost.1;
                new_state.obsidian_bots += 1;
                if !parent.contains(&new_state) {
                    parent.insert(new_state);
                    next.push(new_state);
                }
            }
            if state.ore >= blueprint.geode_cost.0 && state.obsidian >= blueprint.geode_cost.1 {
                let mut new_state = new_state.clone();
                new_state.ore -= blueprint.geode_cost.0;
                new_state.obsidian -= blueprint.geode_cost.1;
                new_state.geode_bots += 1;
                if !parent.contains(&new_state) {
                    parent.insert(new_state);
                    next.push(new_state);
                }
            }
        }

        std::mem::swap(&mut frontier, &mut next);
    }

    *frontier.iter().map(|State { geode, .. }| geode).max().unwrap()
}

pub fn from_iter<T: Iterator, const N: usize>(mut it: T) -> Option<[T::Item; N]> {
    [(); N].try_map(|_| it.next())
}

struct Blueprint {
    ore_cost: u16,
    clay_cost: u16,
    obsidian_cost: (u16, u16),
    geode_cost: (u16, u16),
}

fn parse(input: &str) -> Vec<Blueprint> {
    let number_regex = lazily!(Regex::new(r"\d+").unwrap());

    input
        .lines()
        .map(|line| {
            let numbers = line.matches(number_regex).map(|s| s.parse().unwrap());
            let [_, a, b, c, d, e, f] = from_iter(numbers).unwrap();

            Blueprint {
                ore_cost: a,
                clay_cost: b,
                obsidian_cost: (c, d),
                geode_cost: (e, f),
            }
        })
        .collect()
}
