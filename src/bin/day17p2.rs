#![feature(test)]

extern crate test;

use rustc_hash::FxHashMap;

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input = parse(&input);

    let result = simulate(&input);
    println!("{result}");
}

const N_BLOCKS: usize = 1_000_000_000_000;
const SAFETY: usize = 60;

fn simulate(input: &[Direction]) -> usize {
    let mut movement = input.iter().copied().cycle().enumerate().peekable();

    let mut tower = vec![0u8; 200_000];
    tower[0] = 0xFF;
    let mut height = 1;

    let shapes: [[u8; 4]; 5] = [
        [0b00000000, 0b00000000, 0b00000000, 0b11110000],
        [0b00000000, 0b01000000, 0b11100000, 0b01000000],
        [0b00000000, 0b00100000, 0b00100000, 0b11100000],
        [0b10000000, 0b10000000, 0b10000000, 0b10000000],
        [0b00000000, 0b00000000, 0b11000000, 0b11000000],
    ]
    .map(|mut slice| {
        slice.reverse();
        slice
    });
    let height_of_shape: [usize; 5] = [1, 3, 3, 4, 2];
    let width_of_shape: [usize; 5] = [4, 3, 3, 1, 2];

    fn match_shape(shape: [u8; 4], region: &[u8; 4], left: usize) -> bool {
        (0..4).any(|i| (shape[i] >> left) & region[i] != 0)
    }

    let mut states = FxHashMap::default();

    let mut i = 0;
    let mut extra_height = 0;
    let mut skipped = false;
    while i < N_BLOCKS {
        let block = i % 5;

        if !skipped && block == 0 {
            if let Some((j, old_height)) = states.insert(
                (
                    tower[(height as usize).saturating_sub(SAFETY)..height].to_vec(),
                    movement.peek().unwrap().0 % input.len(),
                ),
                (i, height),
            ) {
                let cycle_length = i - j;
                let left = N_BLOCKS - i;
                let free = left / cycle_length;
                let diff = height - old_height;

                // println!("Found cycle {j}..{i} of length {cycle_length} with diff {diff}, allows skipping {free} cycles");

                extra_height = diff * free;
                i += cycle_length * free;
                skipped = true;
                continue;
            }
        }

        let shape = shapes[block];
        let mut pos = height + 3;
        let mut left = 2;
        let width = width_of_shape[block % 5];

        loop {
            let region: &[u8; 4] = tower[pos..][..4].try_into().unwrap();
            match movement.next().unwrap().1 {
                Direction::Left => {
                    if left > 0 {
                        if !match_shape(shape, region, left - 1) {
                            left -= 1;
                        }
                    }
                }
                Direction::Right => {
                    if left + width < 7 {
                        if !match_shape(shape, region, left + 1) {
                            left += 1;
                        }
                    }
                }
            }

            let region_below: &[u8; 4] = tower[pos - 1..][..4].try_into().unwrap();
            if match_shape(shape, region_below, left) {
                // settle
                for i in 0..4 {
                    tower[pos + i] |= shape[i] >> left;
                }
                break;
            }
            pos -= 1;
        }

        if pos + height_of_shape[block] > height {
            height = pos + height_of_shape[block];
        }

        i += 1;
    }

    // for n in tower[..150].iter().rev() {
    //     for i in (1..8).rev() {
    //         print!("{}", if n & (1 << i) != 0 { '#' } else { ' ' });
    //     }
    //     println!();
    // }

    height + extra_height - 1
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left = -1,
    Right = 1,
}

fn parse(input: &str) -> Vec<Direction> {
    input
        .trim()
        .bytes()
        .map(|ch| match ch {
            b'<' => Direction::Left,
            b'>' => Direction::Right,
            _ => panic!("invalid character in input"),
        })
        .collect()
}

#[bench]
fn bench(bencher: &mut test::Bencher) {
    let input = include_str!("../../inputs/day17");
    let input = parse(input);

    bencher.iter(|| test::black_box(simulate(&input)));
}