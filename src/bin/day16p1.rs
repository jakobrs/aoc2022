#![feature(once_cell)]

use regex::Regex;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

// macro_rules! lazily {
//     ($ty:ty, $expr:expr) => {{
//         static LOCK: ::std::sync::LazyLock<$ty> = ::std::sync::LazyLock::new(|| $expr);

//         &*LOCK
//     }};
// }

macro_rules! lazily {
    ($expr:expr) => {{
        static LAZY: ::std::sync::LazyLock<
            ::std::boxed::Box<
                dyn ::std::any::Any + ::std::marker::Sync + ::std::marker::Send + 'static,
            >,
        > = ::std::sync::LazyLock::new(|| Box::new($expr));

        fn infer_type<T>(_: fn() -> T) -> &'static T {
            LAZY.downcast_ref().unwrap()
        }
        infer_type(|| $expr)
    }};
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = parse(&input);
    // println!("{input:?}");

    println!("graph d {{");
    for (
        name,
        Node {
            neighbours,
            pressure,
            ..
        },
    ) in input.nodes
    {
        if name == "AA" {
            println!("  {name} [label=\"{name}\\n{pressure}\", shape=square, color=darkgreen]");
        } else {
            println!("  {name} [label=\"{name}\\n{pressure}\"]");
        }

        for neighbour in neighbours {
            if name < neighbour {
                println!("  {name} -- {neighbour}");
            }
        }
    }
    println!("}}");
}

fn solve(input: Input) -> usize {
    todo!()
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Node<'a> {
    neighbours: SmallVec<[&'a str; 2]>,
    pressure: u16,
    index: u16,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input<'a> {
    nodes: FxHashMap<&'a str, Node<'a>>,
    number_of_pressure_holding_valves: usize,
}

fn parse(input: &str) -> Input {
    let regex = lazily!(Regex::new(r"[A-Z]{2}").unwrap());
    let digits = lazily!(Regex::new(r"\d+").unwrap());

    let mut nodes = FxHashMap::default();
    let mut index = 0;
    for line in input.lines() {
        let pressure = digits.find(line).unwrap().as_str().parse().unwrap();

        let mut names = line.matches(regex);
        let from = names.next().unwrap();

        nodes.insert(
            from,
            Node {
                neighbours: names.collect(),
                pressure,
                index,
            },
        );

        if pressure > 0 {
            index += 1;
        }
    }

    Input {
        nodes,
        number_of_pressure_holding_valves: index as usize,
    }
}
