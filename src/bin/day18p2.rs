#![feature(test)]

extern crate test;

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
    let result = part2(&input);

    println!("{result:?}");
}

fn part2(input: &Input) -> usize {
    let min = input
        .blocks
        .iter()
        .flat_map(|(x, y, z)| [x, y, z])
        .min()
        .unwrap()
        - 1;
    let max = input
        .blocks
        .iter()
        .flat_map(|(x, y, z)| [x, y, z])
        .max()
        .unwrap()
        + 1;

    let blocks_set: FxHashSet<_> = input.blocks.iter().collect();
    let mut dsu = FxUnionFind::<(i32, i32, i32)>::default();

    for x in min..=max {
        for y in min..=max {
            for z in min..=max {
                if !blocks_set.contains(&(x, y, z)) {
                    if !blocks_set.contains(&(x + 1, y, z)) {
                        dsu.unite_by_key((x, y, z), (x + 1, y, z));
                    }
                    if !blocks_set.contains(&(x, y + 1, z)) {
                        dsu.unite_by_key((x, y, z), (x, y + 1, z));
                    }
                    if !blocks_set.contains(&(x, y, z + 1)) {
                        dsu.unite_by_key((x, y, z), (x, y, z + 1));
                    }
                }
            }
        }
    }

    let outside = dsu.find_by_key((min, min, min));

    let mut sides = FxHashSet::default();

    for x in min..=max {
        for y in min..=max {
            for z in min..=max {
                if !blocks_set.contains(&(x, y, z)) && dsu.find_by_key((x, y, z)) == outside {
                    swap(&mut sides, (2 * x - 1, 2 * y, 2 * z));
                    swap(&mut sides, (2 * x + 1, 2 * y, 2 * z));
                    swap(&mut sides, (2 * x, 2 * y - 1, 2 * z));
                    swap(&mut sides, (2 * x, 2 * y + 1, 2 * z));
                    swap(&mut sides, (2 * x, 2 * y, 2 * z - 1));
                    swap(&mut sides, (2 * x, 2 * y, 2 * z + 1));
                }
            }
        }
    }

    let box_side_length = max - min + 1;
    let outside_box_surface_area = box_side_length.pow(2) * 6;

    sides.len() - outside_box_surface_area as usize
}

fn swap<T: Eq + Hash + Copy, S: BuildHasher>(set: &mut HashSet<T, S>, value: T) {
    if !set.insert(value) {
        set.remove(&value);
    }
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

#[derive(Default)]
pub struct UnionFind<K, S: BuildHasher> {
    node_mapping: HashMap<K, usize, S>,
    parent: Vec<isize>,
}

type FxUnionFind<K> = UnionFind<K, BuildHasherDefault<FxHasher>>;

impl<K: Eq + Hash, S: BuildHasher> UnionFind<K, S> {
    pub fn with_hasher(hash_builder: S) -> Self {
        Self {
            node_mapping: HashMap::with_hasher(hash_builder),
            parent: vec![],
        }
    }

    pub fn find(&mut self, index: usize) -> usize {
        if self.parent[index] < 0 {
            index
        } else {
            let root = self.find(self.parent[index] as usize);
            self.parent[index] = root as isize;
            root
        }
    }

    pub fn find_by_key(&mut self, key: K) -> usize {
        let node = *self.node_mapping.entry(key).or_insert_with(|| {
            self.parent.push(-1);
            self.parent.len() - 1
        });
        self.find(node)
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.find(x);
        let mut y = self.find(y);

        if x == y {
            false
        } else {
            if -self.parent[x] < -self.parent[y] {
                (x, y) = (y, x);
            }

            self.parent[x] += self.parent[y];
            self.parent[y] = x as isize;

            true
        }
    }

    pub fn unite_by_key(&mut self, x: K, y: K) -> bool {
        let x = self.find_by_key(x);
        let y = self.find_by_key(y);
        self.unite(x, y)
    }

    pub fn card(&mut self, x: usize) -> usize {
        let root = self.find(x);
        -self.parent[root] as usize
    }

    pub fn card_by_key(&mut self, x: K) -> usize {
        let root = self.find_by_key(x);
        -self.parent[root] as usize
    }
}

#[bench]
fn bench(bencher: &mut test::Bencher) {
    let input = include_str!("../../inputs/day18");
    let input = parse(input);

    bencher.iter(|| test::black_box(part2(&input)));
}