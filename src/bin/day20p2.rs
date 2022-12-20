#![feature(iterator_try_collect)]
#![feature(test)]

extern crate test;

use std::str::FromStr;

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = parse(&stdin);
    let result = solve(&input);
    println!("{result}");
}

fn solve(input: &[i64]) -> i64 {
    let mut input: Vec<(usize, i64)> = input
        .into_iter()
        .map(|i| i * 811589153)
        .enumerate()
        .collect();

    let n = input.len();

    for _ in 0..10 {
        mix(&mut input);
    }

    let zero = input.iter().position(|&(_, x)| x == 0).unwrap();
    (1..=3)
        .map(|i| input[(zero + 1000 * i).rem_euclid(n)].1)
        .sum()
}

fn mix(input: &mut Vec<(usize, i64)>) {
    let n = input.len();

    for i in 0..input.len() {
        let pos = input.iter().position(|&(j, _)| j == i).unwrap();
        let val = input[pos].1;

        let new_pos = (pos + (val.rem_euclid(n as i64 - 1) as usize)).rem_euclid(n - 1);

        if new_pos < pos {
            input[new_pos..=pos].rotate_right(1);
        } else {
            input[pos..=new_pos].rotate_left(1);
        }
    }
}

fn parse(input: &str) -> Vec<i64> {
    input.lines().map(FromStr::from_str).try_collect().unwrap()
}

#[bench]
fn bench(bencher: &mut test::Bencher) {
    let input = include_str!("../../inputs/day20");
    let input = parse(input);

    bencher.iter(|| {
        let result = solve(&input);
        test::black_box(result)
    })
}
