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
    let mut lines = stdin.lines();

    let mut i = 0;
    let mut sum = 0;

    while let Some(a) = lines.next() {
        let b = lines.next().unwrap();
        i += 1;

        let a_item = from_str(&a);
        let b_item = from_str(&b);

        if a_item < b_item {
            sum += i;
        }

        let _ = lines.next();
    }

    println!("{sum}");

    Ok(())
}
