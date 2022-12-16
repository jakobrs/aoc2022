#![feature(once_cell)]
#![feature(test)]

extern crate test;

use rustc_hash::FxHashSet;

#[derive(Clone, Copy, Debug)]
struct Interval {
    start: isize,
    end: isize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Side {
    Start = 1,
    End = -1,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Endpoint {
    position: isize,
    side: Side,
}

fn count_from_intervals(intervals: &[Interval]) -> usize {
    let mut endpoints: Vec<Endpoint> = intervals
        .iter()
        .flat_map(|&Interval { start, end }| {
            [
                Endpoint {
                    position: start,
                    side: Side::Start,
                },
                Endpoint {
                    position: end,
                    side: Side::End,
                },
            ]
        })
        .collect();

    endpoints.sort_unstable();

    let mut coverage = 0;
    let mut last_pos = 0;
    let mut covered = 0;

    for Endpoint { position, side } in endpoints {
        // println!("{coverage} {last_pos}..{position} {side:?}");
        if coverage > 0 {
            covered += (position - last_pos) as usize;
        }
        coverage += side as isize;
        last_pos = position;
    }

    covered
}

const Y: isize = 2_000_000;

fn find_intervals(input: &str) -> (Vec<Interval>, usize) {
    // static NUMBER_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\-?\d+").unwrap());

    let mut intervals = vec![];
    let mut beacons_in_line = FxHashSet::default();

    for line in input.lines() {
        // let mut numbers = NUMBER_REGEX
        //     .find_iter(line)
        //     .map(|i| i.as_str().parse::<isize>().unwrap());

        let mut numbers = line
            .split(|ch: char| ch != '-' && !ch.is_ascii_digit())
            .filter(|i| !i.is_empty())
            .map(|s| s.parse::<isize>().unwrap());

        let sx = numbers.next().unwrap();
        let sy = numbers.next().unwrap();
        let bx = numbers.next().unwrap();
        let by = numbers.next().unwrap();

        let radius = bx.abs_diff(sx) + by.abs_diff(sy);
        let distance = sy.abs_diff(Y);

        let interval_left_radius = radius.saturating_sub(distance) as isize;
        let interval_right_radius = (radius + 1).saturating_sub(distance) as isize;

        let interval = Interval {
            start: sx - interval_left_radius,
            end: sx + interval_right_radius,
        };
        if interval.start != interval.end {
            intervals.push(interval);
        }

        if by == Y {
            beacons_in_line.insert((bx, by));
        }
    }

    (intervals, beacons_in_line.len())
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    let (intervals, off) = find_intervals(&input);
    // println!("{intervals:#?} {off}");
    let covered = count_from_intervals(&intervals);
    // println!("{covered} {off}");
    let values = covered - off;

    println!("{values}");
}

#[bench]
fn part1_bench(bencher: &mut test::Bencher) {
    let input = include_str!("../../inputs/day15");

    bencher.iter(|| {
        let (intervals, off) = find_intervals(&input);
        let covered = count_from_intervals(&intervals);
        let values = covered - off;

        test::black_box(values)
    })
}

#[bench]
fn find_intervals_bench(bencher: &mut test::Bencher) {
    let input = include_str!("../../inputs/day15");

    bencher.iter(|| {
        let (intervals, off) = find_intervals(&input);

        test::black_box((intervals, off))
    })
}

#[bench]
fn measure_bench(bencher: &mut test::Bencher) {
    let input = include_str!("../../inputs/day15");
    let (intervals, off) = find_intervals(&input);

    bencher.iter(|| {
        let covered = count_from_intervals(&intervals);
        let values = covered - off;

        test::black_box(values)
    })
}
