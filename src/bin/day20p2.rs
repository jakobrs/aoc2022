#![feature(iterator_try_collect)]

use std::str::FromStr;

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = parse(&stdin);

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
    let result: i64 = (1..=3)
        .map(|i| input[(zero + 1000 * i).rem_euclid(n)].1)
        .sum();

    println!("{result}");
}

fn mix(input: &mut Vec<(usize, i64)>) {
    let n = input.len();

    for i in 0..input.len() {
        let pos = input.iter().position(|&(j, _)| j == i).unwrap();
        let (_, val) = input.remove(pos);
        let new_pos = (pos + (val.rem_euclid(n as i64 - 1) as usize)).rem_euclid(n - 1);
        input.insert(new_pos, (i, val));
    }
}

fn parse(input: &str) -> Vec<i64> {
    input.lines().map(FromStr::from_str).try_collect().unwrap()
}
