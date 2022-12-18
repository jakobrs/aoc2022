use std::{
    collections::{HashMap, HashSet},
    hash::{BuildHasher, BuildHasherDefault, Hash},
};

use aoc2022::lazily;
use regex::Regex;
use rustc_hash::{FxHashSet, FxHasher};

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = parse(&input);
    let result = part1(&input);

    println!("{result}");
}

fn swap<T: Eq + Hash + Copy, S: BuildHasher>(set: &mut HashSet<T, S>, value: T) {
    if !set.insert(value) {
        set.remove(&value);
    }
}

fn part1(input: &Input) -> usize {
    let mut sides = FxHashSet::default();

    for &(x, y, z) in &input.blocks {
        swap(&mut sides, (2 * x - 1, 2 * y, 2 * z));
        swap(&mut sides, (2 * x + 1, 2 * y, 2 * z));
        swap(&mut sides, (2 * x, 2 * y - 1, 2 * z));
        swap(&mut sides, (2 * x, 2 * y + 1, 2 * z));
        swap(&mut sides, (2 * x, 2 * y, 2 * z - 1));
        swap(&mut sides, (2 * x, 2 * y, 2 * z + 1));
    }

    sides.len()
}

struct Input {
    blocks: Vec<(i32, i32, i32)>,
}

fn parse(input: &str) -> Input {
    let number_regex = lazily!(Regex::new(r"\d+").unwrap());
    let mut blocks = vec![];

    for line in input.lines() {
        let mut numbers = line.matches(number_regex).map(|a| a.parse().unwrap());

        let a: i32 = numbers.next().unwrap();
        let b: i32 = numbers.next().unwrap();
        let c: i32 = numbers.next().unwrap();

        blocks.push((a, b, c));
    }

    Input { blocks }
}
