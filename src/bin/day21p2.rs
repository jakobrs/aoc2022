#![feature(test)]

use rustc_hash::FxHashMap;

extern crate test;

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let monkeys = parse(&input);

    let answer = solve(&monkeys);

    println!("{answer}");
}

fn solve(monkeys: &FxHashMap<&str, Monkey>) -> i64 {
    fn recurse<'a>(monkeys: &FxHashMap<&'a str, Monkey<'a>>, name: &'a str) -> Val {
        match monkeys[name] {
            Monkey::Int { value } => Val::Int { value },
            Monkey::Human => Val::Human { operations: vec![] },
            Monkey::Operation { op, dep1, dep2 } => {
                let dep1 = recurse(monkeys, dep1);
                let dep2 = recurse(monkeys, dep2);

                match (dep1, dep2) {
                    (Val::Int { value: lhs }, Val::Int { value: rhs }) => Val::Int {
                        value: op.perform(lhs, rhs),
                    },
                    (Val::Human { mut operations }, Val::Int { value }) => {
                        operations.push(HumanOperation { op, rhs: value });
                        Val::Human { operations }
                    }
                    (Val::Int { value }, Val::Human { mut operations }) => {
                        match op {
                            Operation::Plus | Operation::Times => operations.push(HumanOperation { op, rhs: value }),
                            Operation::Minus => {
                                operations.push(HumanOperation { op: Operation::Times, rhs: -1 });
                                operations.push(HumanOperation { op: Operation::Plus, rhs: value });
                            }
                            Operation::Div => panic!("no"),
                        }
                        Val::Human { operations }
                    }
                    (Val::Human { .. }, Val::Human { .. }) => panic!("Human has multiplicity > 1"),
                }
            }
        }
    }

    let Monkey::Operation { dep1, dep2, .. } = &monkeys["root"] else { panic!("root invalid") };
    let lhs = recurse(monkeys, dep1);
    let rhs = recurse(monkeys, dep2);

    let (mut value, operations) = match (lhs, rhs) {
        (Val::Int { value }, Val::Human { operations }) => (value, operations),
        (Val::Human { operations }, Val::Int { value }) => (value, operations),
        (Val::Human { .. }, Val::Human { .. }) => panic!("Human has multiplicity > 1"),
        (Val::Int { .. }, Val::Int { .. }) => panic!("where human"),
    };

    for &HumanOperation { op, rhs } in operations.iter().rev() {
        use Operation::*;
        match op {
            Plus => value -= rhs,
            Minus => value += rhs,
            Times => {
                assert!(value % rhs == 0, "fractional temporaries are unimplemented");
                value /= rhs;
            }
            Div => value *= rhs,
        }
    }

    value
}

enum Monkey<'a> {
    Int {
        value: i64,
    },
    Operation {
        op: Operation,
        dep1: &'a str,
        dep2: &'a str,
    },
    Human,
}

#[derive(Debug)]
enum Val {
    Int { value: i64 },
    Human { operations: Vec<HumanOperation> },
}

#[derive(Debug)]
struct HumanOperation {
    op: Operation,
    rhs: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operation {
    Plus,
    Minus,
    Times,
    Div,
}

impl Operation {
    fn from_str(s: &str) -> Option<Operation> {
        use Operation::*;
        match s {
            "+" => Some(Plus),
            "-" => Some(Minus),
            "*" => Some(Times),
            "/" => Some(Div),
            _ => None,
        }
    }

    fn perform(self, lhs: i64, rhs: i64) -> i64 {
        use Operation::*;
        match self {
            Plus => lhs + rhs,
            Minus => lhs - rhs,
            Times => lhs * rhs,
            Div => lhs / rhs,
        }
    }
}

fn parse(input: &str) -> FxHashMap<&str, Monkey> {
    input
        .lines()
        .map(|line| {
            let name = &line[..4];
            (
                name,
                if name == "humn" {
                    Monkey::Human
                } else if line.len() < 15 {
                    let value = line[6..].parse().unwrap();
                    Monkey::Int { value }
                } else {
                    let op = Operation::from_str(&line[11..12]).unwrap();
                    let dep1 = &line[6..10];
                    let dep2 = &line[13..];
                    Monkey::Operation { op, dep1, dep2 }
                },
            )
        })
        .collect()
}

#[bench]
fn bench(bencher: &mut test::Bencher) {
    let input = include_str!("../../inputs/day21");
    let monkeys = parse(input);

    bencher.iter(|| test::black_box(solve(&monkeys)));
}