use rustc_hash::FxHashMap;

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let monkeys = parse(&input);

    println!("digraph {{");
    for (k, v) in monkeys {
        match v {
            Monkey::Int { value } => println!("  {k} [label={value}]"),
            Monkey::Operation { op, dep1, dep2 } => {
                println!("  {k} [label={op}]", op = op.as_str());
                println!("  {k} -> {dep1}");
                println!("  {k} -> {dep2}");
            }
            Monkey::Human { value } => {
                println!("  {k} [label=Human ({value}),color=darkgreen,shape=rectangle]")
            }
        }
    }
    println!("}}");
}

#[derive(Clone, Copy)]
enum Monkey<'a> {
    Int {
        value: i64,
    },
    Human {
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

    fn as_str(self) -> &'static str {
        use Operation::*;
        match self {
            Plus => "+",
            Minus => "-",
            Times => "*",
            Div => "/",
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
                    let value = line[6..].parse().unwrap();
                    Monkey::Human { value }
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
