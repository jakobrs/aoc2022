#![feature(test)]

extern crate test;

use std::collections::VecDeque;

use strength_reduce::StrengthReducedUsize;

type Node = (usize, usize);

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();

    let answer = inner(&stdin);

    println!("{answer:?}");
}

fn inner(stdin: &str) -> Option<usize> {
    let height = stdin.matches('\n').count();
    let width = stdin.find('\n').unwrap() + 1;

    let width_reduced = StrengthReducedUsize::new(width);

    let to_coords = |n: usize| -> Node { (n / width_reduced, n % width_reduced) };
    // let to_index = |(r, c): Node| -> usize { r * width + c };

    let start_pos = stdin.find('S').unwrap();
    let end_pos = stdin.find('E').unwrap();

    let mut grid = stdin.as_bytes().to_owned();

    grid[start_pos] = b'a';
    grid[end_pos] = b'z';

    let mut queue = VecDeque::new();
    queue.push_back((end_pos, 0, b'z'));

    while let Some((pos, dist, elevation)) = queue.pop_front() {
        let node = to_coords(pos);

        if elevation == b'a' {
            return Some(dist);
        }

        let dist = dist + 1;
        let elevation = elevation - 1;

        macro_rules! attempt {
            ($cond:expr, $new_pos:expr) => {
                if $cond {
                    let new_pos = $new_pos;
                    let new_elevation = grid[new_pos];

                    if new_elevation >= elevation {
                        grid[new_pos] = b'a' - 2;

                        queue.push_back((new_pos, dist, new_elevation));
                    }
                }
            };
        }

        attempt!(node.0 > 0, pos - width);
        attempt!(node.1 > 0, pos - 1);
        attempt!(node.0 + 1 < height, pos + width);
        attempt!(node.1 + 1 < width - 1, pos + 1);
    }

    None
}

#[bench]
fn bench_thingy(bencher: &mut test::Bencher) {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();

    bencher.iter(|| {
        let result = inner(&stdin);

        test::black_box(result)
    });
}
