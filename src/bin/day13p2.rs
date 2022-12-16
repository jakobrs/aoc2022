#![feature(test)]

extern crate test;

use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Item {
    Number(u64),
    List(Vec<Item>),
}

impl Ord for Item {
    fn cmp(&self, right: &Item) -> Ordering {
        use Item::*;

        match (self, right) {
            (Number(a), Number(b)) => a.cmp(b),
            (List(a), List(b)) => {
                for i in 0..a.len().min(b.len()) {
                    let ord = a[i].cmp(&b[i]);

                    if ord != Ordering::Equal {
                        return ord;
                    }
                }

                a.len().cmp(&b.len())
            }
            (a @ Number(_), b @ List(_)) => List(vec![a.clone()]).cmp(&b),
            (a @ List(_), b @ Number(_)) => a.cmp(&List(vec![b.clone()])),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn from_str(text: &str) -> Item {
    use Item::*;

    let mut contexts: Vec<Vec<Item>> = vec![vec![]];
    let mut n = None;

    for ch in text.bytes() {
        // termination tokens
        if matches!(ch, b']' | b',') {
            if let Some(n) = n.take() {
                contexts.last_mut().unwrap().push(Number(n));
            }
        }

        match ch {
            b'[' => contexts.push(vec![]),
            b']' => {
                let list = contexts.pop().unwrap();
                contexts.last_mut().unwrap().push(List(list));
            }
            b',' => (), // handled above
            ch if ch.is_ascii_digit() => {
                if n.is_none() {
                    n = Some(0);
                }
                let Some(ref mut n) = n else { panic!("uwu") };
                *n *= 10;
                *n += (ch as u8 - b'0') as u64;
            }
            _ => panic!("wot {ch}"),
        }
    }

    contexts.pop().unwrap().pop().unwrap()
}

use anyhow::Result;

fn main() -> Result<()> {
    let stdin = std::io::read_to_string(std::io::stdin())?;

    let sum = inner(&stdin);
    println!("{sum}");

    Ok(())
}

fn inner(stdin: &str) -> usize {
    let packets: Vec<Item> = stdin
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(from_str(line))
            }
        })
        .collect();

    let two_position = packets
        .iter()
        .filter(|item| item < &&Item::Number(2))
        .count()
        + 1;
    let six_position = packets
        .iter()
        .filter(|item| item < &&Item::Number(6))
        .count()
        + 2;

    two_position * six_position
}

#[bench]
fn benchmarks(bencher: &mut test::Bencher) {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();

    bencher.iter(|| {
        let result = inner(&stdin);

        test::black_box(result)
    });
}
