#![feature(test)]
#![feature(generators)]
#![feature(iter_from_generator)]

extern crate test;

use std::collections::VecDeque;

use aoc2022::lazily;
use regex::Regex;

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = parse(&input);
    let result = part2(&input);

    println!("{result:?}");
}

type Point = [usize; 3];

fn part2(input: &Input) -> usize {
    let min = input.blocks.iter().flatten().min().unwrap() - 1;
    let max = input.blocks.iter().flatten().max().unwrap() + 1;

    let side_length = (max - min + 1) as usize;
    let to_index = |[x, y, z]: Point| x + side_length * (y + side_length * z);

    let mut blocks_set = vec![false; side_length * side_length * side_length];
    let mut visited = vec![false; side_length * side_length * side_length];

    for point in &input.blocks {
        blocks_set[to_index(point.map(|i| (i - min) as usize))] = true;
    }

    let mut queue = VecDeque::new();
    queue.push_back([0, 0, 0]);

    #[rustfmt::skip]
    let neighbours = |[x, y, z]: Point| std::iter::from_generator(move || {
        if x > 0 { yield [x - 1, y, z] }
        if y > 0 { yield [x, y - 1, z] }
        if z > 0 { yield [x, y, z - 1] }

        if x + 1 < side_length { yield [x + 1, y, z] }
        if y + 1 < side_length { yield [x, y + 1, z] }
        if z + 1 < side_length { yield [x, y, z + 1] }
    });

    let mut area = 0;

    while let Some(point) = queue.pop_front() {
        for neighbour in neighbours(point) {
            if blocks_set[to_index(neighbour)] {
                area += 1;
            } else {
                if !visited[to_index(neighbour)] {
                    queue.push_back(neighbour);
                    visited[to_index(neighbour)] = true;
                }
            }
        }
    }

    area
}

struct Input {
    blocks: Vec<[i32; 3]>,
}

fn parse(input: &str) -> Input {
    let number_regex = lazily!(Regex::new(r"\d+").unwrap());
    let mut blocks = vec![];

    for line in input.lines() {
        let mut numbers = line.matches(number_regex).map(|a| a.parse().unwrap());

        let a: i32 = numbers.next().unwrap();
        let b: i32 = numbers.next().unwrap();
        let c: i32 = numbers.next().unwrap();

        blocks.push([a, b, c]);
    }

    Input { blocks }
}

#[bench]
fn bench(bencher: &mut test::Bencher) {
    let input = include_str!("../../inputs/day18");
    let input = parse(input);

    bencher.iter(|| test::black_box(part2(&input)));
}
