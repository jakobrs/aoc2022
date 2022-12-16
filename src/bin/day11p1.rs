use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisor: u64,
    if_true: usize,
    if_false: usize,
    count: usize,
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

fn find_numbers<'a, T: FromStr>(line: &'a str) -> impl Iterator<Item = T> + 'a
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
    let mut lines = stdin.lines();

    let mut monkeys = vec![];

    while let Some(_line) = lines.next() {
        let items: Vec<u64> = find_numbers(lines.next().unwrap()).collect();

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
            count: 0,

            items,
            operation,
            divisor,
            if_true,
            if_false,
        });
    }

    println!("Monkeys: {monkeys:#?}");

    for _round in 0..20 {
        for monkey in 0..monkeys.len() {
            let Monkey {
                operation,
                divisor,
                if_true,
                if_false,
                ref mut items,
                ref mut count,
            } = monkeys[monkey];

            let mut if_true_set = vec![];
            let mut if_false_set = vec![];

            *count += items.len();

            for item in items.drain(..) {
                let new_stress_level = operation.apply(item) / 3;

                if new_stress_level % divisor == 0 {
                    if_true_set.push(new_stress_level);
                } else {
                    if_false_set.push(new_stress_level);
                }
            }

            monkeys[if_true].items.extend(if_true_set);
            monkeys[if_false].items.extend(if_false_set);
        }
    }

    monkeys.sort_by_key(|&Monkey { count, .. }| count);

    let business: usize = monkeys
        .iter()
        .rev()
        .take(2)
        .map(|&Monkey { count, .. }| count)
        .product();

    println!("{business}");
}
