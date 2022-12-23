#![feature(test)]

extern crate test;

use rustc_hash::FxHashSet;

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    let result = solve(&input);
    println!("{result}");
}

fn solve(input: &str) -> usize {
    // (row, column)
    let mut elves = FxHashSet::default();
    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == '#' {
                elves.insert((r as i64, c as i64));
            }
        }
    }

    let mut new_pos = FxHashSet::default();
    for i in 0.. {
        'elves: for &(r, c) in &elves {
            let mut should_move = false;
            'outer: for rd in -1..=1 {
                for cd in -1..=1 {
                    if (rd, cd) == (0, 0) {
                        continue;
                    }
                    if elves.contains(&(r + rd, c + cd)) {
                        should_move = true;
                        break 'outer;
                    }
                }
            }
            if should_move {
                for direction in i..i + 4 {
                    let direction = direction % 4;

                    match direction {
                        0 => {
                            if !(elves.contains(&(r - 1, c))
                                || elves.contains(&(r - 1, c - 1))
                                || elves.contains(&(r - 1, c + 1)))
                            {
                                if !new_pos.insert((r - 1, c)) {
                                    new_pos.remove(&(r - 1, c));
                                    new_pos.insert((r - 2, c));
                                    new_pos.insert((r, c));
                                }
                                continue 'elves;
                            }
                        }
                        1 => {
                            if !(elves.contains(&(r + 1, c))
                                || elves.contains(&(r + 1, c - 1))
                                || elves.contains(&(r + 1, c + 1)))
                            {
                                if !new_pos.insert((r + 1, c)) {
                                    new_pos.remove(&(r + 1, c));
                                    new_pos.insert((r + 2, c));
                                    new_pos.insert((r, c));
                                }
                                continue 'elves;
                            }
                        }
                        2 => {
                            if !(elves.contains(&(r, c - 1))
                                || elves.contains(&(r - 1, c - 1))
                                || elves.contains(&(r + 1, c - 1)))
                            {
                                if !new_pos.insert((r, c - 1)) {
                                    new_pos.remove(&(r, c - 1));
                                    new_pos.insert((r, c - 2));
                                    new_pos.insert((r, c));
                                }
                                continue 'elves;
                            }
                        }
                        3 => {
                            if !(elves.contains(&(r, c + 1))
                                || elves.contains(&(r - 1, c + 1))
                                || elves.contains(&(r + 1, c + 1)))
                            {
                                if !new_pos.insert((r, c + 1)) {
                                    new_pos.remove(&(r, c + 1));
                                    new_pos.insert((r, c + 2));
                                    new_pos.insert((r, c));
                                }
                                continue 'elves;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            new_pos.insert((r, c));
        }

        if new_pos == elves {
            return i + 1;
        }

        std::mem::swap(&mut elves, &mut new_pos);
        new_pos.clear();
    }

    unreachable!()
}

fn vis(elves: &FxHashSet<(i64, i64)>) {
    let random_elf = elves.iter().next().unwrap();
    let mut min_r = random_elf.0;
    let mut max_r = random_elf.0;
    let mut min_c = random_elf.1;
    let mut max_c = random_elf.1;

    for elf in elves.iter().skip(1) {
        min_r = min_r.min(elf.0);
        max_r = max_r.max(elf.0);
        min_c = min_c.min(elf.1);
        max_c = max_c.max(elf.1);
    }

    let h = (max_r - min_r + 1) as usize;
    let w = (max_c - min_c + 1) as usize;

    let mut grid = vec![vec![b'.'; w]; h];

    for &(r, c) in elves {
        grid[(r - min_r) as usize][(c - min_c) as usize] = b'#';
    }

    for row in grid {
        println!("{}", std::str::from_utf8(&row).unwrap());
    }
}

#[bench]
fn bench(bencher: &mut test::Bencher) {
    let input = include_str!("../../inputs/day23");

    bencher.iter(|| test::black_box(solve(input)));
}
