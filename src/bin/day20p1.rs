#![feature(iterator_try_collect)]

use std::str::FromStr;

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = parse(&stdin);

    let mut input: Vec<(usize, i32)> = input.into_iter().enumerate().collect();

    let n = input.len();

    for i in 0..input.len() {
        let pos = input.iter().position(|&(j, _)| j == i).unwrap();
        let (_, val) = input.remove(pos);
        let new_pos = (pos + (val.rem_euclid(n as i32 - 1) as usize)).rem_euclid(n - 1);
        input.insert(new_pos, (i, val));
    }

    // println!("{:?}", input.iter().map(|(_, j)| *j).collect::<Vec<_>>());

    let zero = input.iter().position(|&(_, x)| x == 0).unwrap();
    let result: i32 = (1..=3).map(|i| input[(zero + 1000*i).rem_euclid(n)].1).sum();
    println!("{result}");
}

fn parse(input: &str) -> Vec<i32> {
    input.lines().map(FromStr::from_str).try_collect().unwrap()
}
