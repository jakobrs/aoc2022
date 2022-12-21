use rustc_hash::FxHashMap;

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let monkeys = parse(&input);

    let answer = solve(&monkeys);

    println!("{answer}");
}

fn solve<'a>(monkeys: &FxHashMap<&'a str, Monkey<'a>>) -> i64 {
    fn recurse<'b>(monkeys: &FxHashMap<&'b str, Monkey<'b>>, name: &'b str) -> i64 {
        match monkeys[name] {
            Monkey::Int { value } => value,
            Monkey::Operation { op, dep1, dep2 } => {
                let dep1 = recurse(monkeys, dep1);
                let dep2 = recurse(monkeys, dep2);
                let result = op.perform(dep1, dep2);
                // monkeys.insert(name, Monkey::Int { value: result });
                result
            }
        }
    }

    recurse(monkeys, "root")
}

#[derive(Clone, Copy)]
enum Monkey<'a> {
    Int {
        value: i64,
    },
    Operation {
        op: Operation,
        dep1: &'a str,
        dep2: &'a str,
    },
}

#[derive(Clone, Copy, Debug)]
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
                if line.len() < 15 {
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
