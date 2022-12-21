#![feature(test)]
#![feature(array_zip)]

use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use rustc_hash::FxHashMap;

extern crate test;

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let monkeys = parse(&input);

    let answer = solve(&monkeys);

    println!("{answer}");
}

fn solve(monkeys: &FxHashMap<&str, Monkey>) -> i64 {
    fn recurse<'a>(monkeys: &FxHashMap<&'a str, Monkey<'a>>, name: &'a str) -> Polynomial<2> {
        match monkeys[name] {
            Monkey::Polynomial(polynomial) => polynomial,
            Monkey::Operation { op, dep1, dep2 } => {
                let dep1 = recurse(monkeys, dep1);
                let dep2 = recurse(monkeys, dep2);

                match op {
                    Operation::Plus => dep1 + dep2,
                    Operation::Minus => dep1 - dep2,
                    Operation::Times => dep1 * dep2,
                    Operation::Div => {
                        if dep2.coefficients[1] == 0. {
                            dep1 / dep2.coefficients[0]
                        } else {
                            panic!("no")
                        }
                    }
                }
            }
        }
    }

    let polynomial = recurse(monkeys, "root");

    (-polynomial.coefficients[0] / polynomial.coefficients[1]).round() as i64
}

enum Monkey<'a> {
    Polynomial(Polynomial<2>),
    Operation {
        op: Operation,
        dep1: &'a str,
        dep2: &'a str,
    },
}

#[derive(Debug, Clone, Copy)]
struct Polynomial<const DEGREE: usize> {
    coefficients: [f64; DEGREE],
}

impl<const N: usize> Display for Polynomial<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for (i, &n) in self.coefficients.iter().enumerate() {
            if n == 0. {
                continue;
            }
            if first {
                first = false;
                if n < 0. {
                    f.write_str("-")?;
                }
            } else {
                if n < 0. {
                    f.write_str(" - ")?;
                } else {
                    f.write_str(" + ")?;
                }
            }
            let n = n.abs();

            if n == 1. {
                match i {
                    0 => write!(f, "1"),
                    1 => write!(f, "x"),
                    _ => write!(f, "x^{i}"),
                }
            } else {
                match i {
                    0 => write!(f, "{n}"),
                    1 => write!(f, "{n} x"),
                    _ => write!(f, "{n} x^{i}"),
                }
            }?;
        }

        Ok(())
    }
}

impl<const N: usize> Polynomial<N> {
    fn pointwise(self, rhs: Self, mut op: impl FnMut(f64, f64) -> f64) -> Self {
        Self {
            coefficients: self
                .coefficients
                .zip(rhs.coefficients)
                .map(|(a, b)| op(a, b)),
        }
    }
}

impl<const N: usize> Add<Polynomial<N>> for Polynomial<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.pointwise(rhs, |a, b| a + b)
    }
}

impl<const N: usize> Sub<Polynomial<N>> for Polynomial<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.pointwise(rhs, |a, b| a - b)
    }
}

impl Mul<Polynomial<2>> for Polynomial<2> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            coefficients: [
                self.coefficients[0] * rhs.coefficients[0],
                self.coefficients[1] * rhs.coefficients[0]
                    + self.coefficients[0] * rhs.coefficients[1],
            ],
        }
    }
}

impl<const N: usize> Mul<f64> for Polynomial<N> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            coefficients: self.coefficients.map(|n| n * rhs),
        }
    }
}

impl<const N: usize> Div<f64> for Polynomial<N> {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            coefficients: self.coefficients.map(|n| {
                // assert!(n % rhs == 0);
                n / rhs
            }),
        }
    }
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
}

fn parse(input: &str) -> FxHashMap<&str, Monkey> {
    input
        .lines()
        .map(|line| {
            let name = &line[..4];
            (
                name,
                if name == "humn" {
                    Monkey::Polynomial(Polynomial {
                        coefficients: [0., 1.],
                    })
                } else if line.len() < 15 {
                    let value: f64 = line[6..].parse().unwrap();
                    Monkey::Polynomial(Polynomial {
                        coefficients: [value, 0.],
                    })
                } else {
                    let op = if name == "root" {
                        Operation::Minus
                    } else {
                        Operation::from_str(&line[11..12]).unwrap()
                    };
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