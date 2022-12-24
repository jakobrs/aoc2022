#![feature(generators)]
#![feature(iter_from_generator)]

use anyhow::Result;
use rustc_hash::{FxHashSet, FxHashMap};

fn main() -> Result<()> {
    let stdin = std::io::read_to_string(std::io::stdin())?.into_bytes();
    let width = stdin.iter().position(|&ch| ch == b'\n').unwrap();
    let height = (stdin.len() + 1) / (width + 1);
    let h_per = width - 2;
    let v_per = height - 2;

    let buf = v_per * h_per * 1000;

    println!("{width} {height}");

    let neighbours = |r: usize, c: usize| {
        [(r - 1, c), (r, c - 1), (r + 1, c), (r, c + 1), (r, c)]
            .into_iter()
            .filter(|&(r, c)| r >= 1 && c >= 1 && r <= height - 2 && c <= width - 2)
    };

    let normalise = |r: usize, c: usize| {
        let r_norm = (r - 1) % v_per + 1;
        let c_norm = (c - 1) % h_per + 1;
        (r_norm, c_norm)
    };
    let read = |r: usize, c: usize| {
        let (r, c) = normalise(r, c);
        stdin[r * (width + 1) + c]
    };

    let mut frontier = vec![];
    let mut next = vec![];
    let mut marked = FxHashSet::default();

    let mut solution = 0;
    'outer: for t in 1.. {
        frontier.push((0, 1));

        while let Some((r, c)) = frontier.pop() {
            if r == height - 2 && c == width - 2 {
                solution = t;
                break 'outer;
            }

            for (r1, c1) in neighbours(r, c) {
                if !marked.insert((r1, c1)) {
                    continue;
                }

                // this could've been a single expression but rustfmt didn't like it ...
                let valid = 'a: {
                    if read(r1 + t, c1) == b'^' {
                        break 'a false;
                    }
                    if read(r1, c1 + t) == b'<' {
                        break 'a false;
                    }
                    if read(r1 + buf - t, c1) == b'v' {
                        break 'a false;
                    }
                    if read(r1, c1 + buf - t) == b'>' {
                        break 'a false;
                    }
                    true
                };

                if valid {
                    next.push((r1, c1));
                }
            }
        }

        std::mem::swap(&mut frontier, &mut next);
        next.clear();
        marked.clear();
    }

    println!("{solution}");

    Ok(())
}
