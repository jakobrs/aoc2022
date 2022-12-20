#![feature(iterator_try_collect)]

use std::str::FromStr;

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = parse(&stdin);

    let mut input: Vec<(usize, i32)> = input.into_iter().enumerate().collect();

    let n = input.len();

    for i in 0..input.len() {
        let pos = input.iter().position(|&(j, _)| j == i).unwrap();
        let val = input[pos].1;

        let new_pos = (pos + (val.rem_euclid(n as i32 - 1) as usize)).rem_euclid(n - 1);

        if new_pos < pos {
            input[new_pos..=pos].rotate_right(1);
        } else {
            input[pos..=new_pos].rotate_left(1);
        }
    }

    // println!("{:?}", input.iter().map(|(_, j)| *j).collect::<Vec<_>>());

    let zero = input.iter().position(|&(_, x)| x == 0).unwrap();
    let result: i32 = (1..=3)
        .map(|i| input[(zero + 1000 * i).rem_euclid(n)].1)
        .sum();
    println!("{result}");
}

fn parse(input: &str) -> Vec<i32> {
    input.lines().map(FromStr::from_str).try_collect().unwrap()
}
