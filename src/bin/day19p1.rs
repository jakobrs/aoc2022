#![feature(array_try_map)]

use std::collections::VecDeque;

use aoc2022::lazily;
use regex::Regex;
use rustc_hash::FxHashMap;

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let blueprints = parse(&input);

    let n: usize = blueprints
        .iter()
        .enumerate()
        .map(|(i, blueprint)| {
            let max = simulate(blueprint);
            (i + 1) * max as usize
        })
        .sum();
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
    let mut states = VecDeque::new();
    let mut parent = FxHashMap::default();

    states.push_back((0, State::default()));

    let mut best = State::default();

    let mut best_time = 0;

    while let Some((time, state)) = states.pop_front() {
        if state.geode > best.geode {
            best = state;
        }

        if time > best_time {
            best_time = time;
            println!("{time}");
        }

        if time == 24 {
            continue;
        }

        let mut new_state = state.clone();
        new_state.ore += state.ore_bots;
        new_state.clay += state.clay_bots;
        new_state.obsidian += state.obsidian_bots;
        new_state.geode += state.geode_bots;

        if !parent.contains_key(&new_state) {
            parent.insert(new_state, state);
            states.push_back((time + 1, new_state));
        }

        if state.ore_bots < 4 && state.ore >= blueprint.ore_cost {
            let mut new_state = new_state.clone();
            new_state.ore -= blueprint.ore_cost;
            new_state.ore_bots += 1;
            if !parent.contains_key(&new_state) {
                parent.insert(new_state, state);
                states.push_back((time + 1, new_state));
            }
        }
        if state.clay_bots < 20 && state.ore >= blueprint.clay_cost {
            let mut new_state = new_state.clone();
            new_state.ore -= blueprint.clay_cost;
            new_state.clay_bots += 1;
            if !parent.contains_key(&new_state) {
                parent.insert(new_state, state);
                states.push_back((time + 1, new_state));
            }
        }
        if state.obsidian_bots < 20
            && state.ore >= blueprint.obsidian_cost.0
            && state.clay >= blueprint.obsidian_cost.1
        {
            let mut new_state = new_state.clone();
            new_state.ore -= blueprint.obsidian_cost.0;
            new_state.clay -= blueprint.obsidian_cost.1;
            new_state.obsidian_bots += 1;
            if !parent.contains_key(&new_state) {
                parent.insert(new_state, state);
                states.push_back((time + 1, new_state));
            }
        }
        if state.ore >= blueprint.geode_cost.0 && state.obsidian >= blueprint.geode_cost.1 {
            let mut new_state = new_state.clone();
            new_state.ore -= blueprint.geode_cost.0;
            new_state.obsidian -= blueprint.geode_cost.1;
            new_state.geode_bots += 1;
            if !parent.contains_key(&new_state) {
                parent.insert(new_state, state);
                states.push_back((time + 1, new_state));
            }
        }
    }

    let mut here = best;
    loop {
        println!("{here:?}");
        if let Some(new_here) = parent.get(&here) {
            here = *new_here;
        } else {
            break;
        }
    }

    best.geode
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
