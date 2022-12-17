#![feature(test)]

extern crate test;

use std::{collections::HashMap, str::FromStr, time::Instant};

use once_cell::sync::Lazy;
use regex::Regex;
use strength_reduce::StrengthReducedU64;

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

#[derive(Debug)]
struct Monkey {
    operation: Operation,
    divisor: u64,
    if_true: usize,
    if_false: usize,
}

impl Operation {
    fn apply(&self, item: u64) -> u64 {
        match self {
            Operation::Add(n) => item + n,
            Operation::Multiply(n) => item * n,
            Operation::Square => item * item,
        }
    }
}

fn find_numbers<T: FromStr>(line: &str) -> impl Iterator<Item = T> + '_
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    static NUMBER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

    NUMBER_REGEX
        .find_iter(line)
        .map(|i| i.as_str().parse().unwrap())
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let (monkeys, items) = parse(&stdin);
    let business = solve(&monkeys, &items);

    println!("{business}");
}

fn parse(stdin: &str) -> (Vec<Monkey>, Vec<(u64, usize)>) {
    let mut lines = stdin.lines();

    let mut monkeys = vec![];
    let mut items = vec![];

    let mut e = 0;
    while let Some(_line) = lines.next() {
        items.extend(find_numbers(lines.next().unwrap()).map(|i| (i, e)));
        e += 1;

        let operation_line = lines.next().unwrap();

        let operation = if operation_line.contains("new = old * old") {
            Operation::Square
        } else {
            let n = find_numbers(operation_line).next().unwrap();

            if operation_line.contains("*") {
                Operation::Multiply(n)
            } else {
                Operation::Add(n)
            }
        };

        let divisor = find_numbers(lines.next().unwrap()).next().unwrap();

        let if_true = find_numbers(lines.next().unwrap()).next().unwrap();
        let if_false = find_numbers(lines.next().unwrap()).next().unwrap();

        lines.next();

        monkeys.push(Monkey {
            operation,
            divisor,
            if_true,
            if_false,
        });
    }

    (monkeys, items)
}

fn solve(monkeys: &[Monkey], items: &[(u64, usize)]) -> usize {
    let modulus: u64 = monkeys
        .iter()
        .map(|&Monkey { divisor, .. }| divisor)
        .product();

    let modulus_reduced = strength_reduce::StrengthReducedU64::new(modulus);

    let reduced_divisors: Vec<StrengthReducedU64> =
        (1..20).map(|i| StrengthReducedU64::new(i)).collect();

    let mut count = [0; 8];

    const ROUNDS: usize = 10_000;

    std::thread::scope(|s| {
        let handles: Vec<_> = items
            .into_iter()
            .map(|(mut item, mut monkey)| {
                let monkeys = &monkeys;
                let reduced_divisors = &reduced_divisors;

                s.spawn(move || {
                    let mut round = 0;

                    let mut count = [0; 8];

                    while round < ROUNDS {
                        count[monkey] += 1;

                        let Monkey {
                            operation,
                            divisor,
                            if_true,
                            if_false,
                        } = monkeys[monkey];

                        item = operation.apply(item) % modulus_reduced;

                        let new_monkey = if item % reduced_divisors[divisor as usize - 1] == 0 {
                            if_true
                        } else {
                            if_false
                        };

                        if new_monkey < monkey {
                            round += 1;
                        }
                        monkey = new_monkey;
                    }

                    count
                })
            })
            .collect();

        for handle in handles {
            let local_count = handle.join().unwrap();

            for i in 0..8 {
                count[i] += local_count[i];
            }
        }
    });

    count.sort();

    count.iter().rev().take(2).product()
}

#[bench]
fn bench(bencher: &mut test::Bencher) {
    let input = include_str!("../../inputs/day11");
    let (monkeys, items) = parse(&input);

    bencher.iter(|| {
        test::black_box(solve(&monkeys, &items))
    })
}