#![feature(test)]
#![feature(generators)]
#![feature(iter_from_generator)]

extern crate test;

use aoc2022::lazily;
use regex::Regex;
use rustc_hash::FxHashSet;

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

    // let mut visited = vec![false; side_length * side_length * side_length];
    let mut visited = FxHashSet::default();
    let mut stack = vec![(min, min, min)];

    let neighbours = |(x, y, z)| std::iter::from_generator(move || {
        if x > min { yield (x - 1, y, z) }
        if y > min { yield (x, y - 1, z) }
        if z > min { yield (x, y, z - 1) }

        if x < max { yield (x + 1, y, z) }
        if y < max { yield (x, y + 1, z) }
        if z < max { yield (x, y, z + 1) }
    });

    let mut area = 0;

    while let Some(point) = stack.pop() {
        if !visited.insert(point) {
            continue;
        }

        for neighbour in neighbours(point) {
            if blocks_set.contains(&neighbour) {
                area += 1;
            } else {
                stack.push(neighbour);
            }
        }
    }

    area
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

#[bench]
fn bench(bencher: &mut test::Bencher) {
    let input = include_str!("../../inputs/day18");
    let input = parse(input);

    bencher.iter(|| test::black_box(part2(&input)));
}