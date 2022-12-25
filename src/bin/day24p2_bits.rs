#![feature(test)]

extern crate test;

use anyhow::Result;

fn main() -> Result<()> {
    let stdin = std::io::read_to_string(std::io::stdin())?;

    println!("{}", solve(&stdin));
    Ok(())
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn solve(stdin: &str) -> usize {
    let width = stdin.chars().position(|ch| ch == '\n').unwrap();
    let height = (stdin.len() + 1) / (width + 1);
    let h_per = width - 2;
    let v_per = height - 2;

    let period = v_per * h_per / gcd(v_per, h_per);
    let buf = period * 1000;

    let mut down = vec![0u64; h_per];
    let mut up = vec![0u64; h_per];
    let mut left = vec![0u64; h_per];
    let mut right = vec![0u64; h_per];

    for (r, line) in stdin.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            (match ch {
                'v' => &mut down,
                '^' => &mut up,
                '<' => &mut left,
                '>' => &mut right,
                _ => continue,
            }[c - 1]) |= 1 << (r - 1);
        }
    }

    let fastest_from_to = |start: usize, side: bool, to: (usize, usize)| {
        let mut grid = vec![0u64; h_per];
        let mut new_grid = vec![0u64; h_per];

        for j in (start + 1).. {
            for i in 0..h_per {
                new_grid[i] = grid[i] | grid[i] << 1 | grid[i] >> 1;
                if i > 0 {
                    new_grid[i] |= grid[i - 1];
                } else if side == false {
                    new_grid[i] |= 1;
                }
                if i < h_per - 1 {
                    new_grid[i] |= grid[i + 1];
                } else if side == true {
                    new_grid[i] |= 1 << (v_per - 1);
                }
                new_grid[i] &= !(up[i] >> (j % v_per) | up[i] << (v_per - j % v_per));
                new_grid[i] &= !(down[i] << (j % v_per) | down[i] >> (v_per - j % v_per));
                new_grid[i] &= !(left[(i + j) % h_per]);
                new_grid[i] &= !(right[(i + buf - j) % h_per]);
                new_grid[i] &= (1 << v_per) - 1;
            }

            if new_grid[to.1] & (1 << to.0) != 0 {
                return j + 1;
            }

            std::mem::swap(&mut grid, &mut new_grid);
        }

        unreachable!()
    };

    let part1 = fastest_from_to(0, false, (v_per - 1, h_per - 1));
    let part2 = fastest_from_to(part1, true, (0, 0));
    let part3 = fastest_from_to(part2, false, (v_per - 1, h_per - 1));

    part3
}

#[bench]
fn bench(bencher: &mut test::Bencher) {
    let input = include_str!("../../inputs/day24");

    bencher.iter(|| test::black_box(solve(input)));
}
