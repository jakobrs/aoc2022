#![feature(test)]
#![feature(iter_array_chunks)]
#![feature(slice_flatten)]

extern crate test;

use std::{
    io::Write,
    ops::{Index, IndexMut},
    process::{Child, ChildStdin, Command, Stdio},
};

use bitvec::prelude::*;
use regex::Regex;
use test::Bencher;

const WIDTH: usize = 1000;
const HEIGHT: usize = 200;

#[derive(Clone)]
struct Grid {
    // cells: Box<[bool; HEIGHT * WIDTH]>,
    cells: Box<BitArray<[u8; WIDTH * HEIGHT / 8], Lsb0>>,
}

type Point = (usize, usize);

impl Grid {
    fn to_index((r, c): Point) -> usize {
        r * WIDTH + c
    }

    fn set(&mut self, point: Point, value: bool) {
        self.cells.set(Self::to_index(point), value);
    }
}

impl Index<Point> for Grid {
    type Output = bool;

    fn index(&self, point: Point) -> &Self::Output {
        &self.cells[Self::to_index(point)]
    }
}

// impl IndexMut<(usize, usize)> for Grid {
//     fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut Self::Output {
//         &mut self.cells[r * WIDTH + c]
//     }
// }

struct Display {
    frame: [[[u8; 3]; WIDTH]; HEIGHT],
    child: Child,
    file: ChildStdin,
}

impl Display {
    fn new() -> Self {
        let video_size = format!("{WIDTH}x{HEIGHT}");
        let mut child = Command::new("ffmpeg")
            .args([
                "-loglevel",
                "warning",
                "-stats",
                "-f",
                "rawvideo",
                "-pixel_format",
                "rgb24",
                "-video_size",
                &video_size,
                "-framerate",
                "30",
                "-i",
                "-",
                "-vf",
                "crop=200:200:400:200,scale=iw*4:-1:flags=neighbor",
                "-pix_fmt",
                "yuv420p",
                "output.mp4",
            ])
            .stdin(Stdio::piped())
            .spawn()
            .unwrap();

        let file = child.stdin.take().unwrap();

        Self {
            frame: [[[0; 3]; WIDTH]; HEIGHT],
            child,
            file,
        }
    }

    fn emit_frame(&mut self) -> std::io::Result<()> {
        self.file.write_all(self.frame.flatten().flatten())
    }
}

fn drop(display: &mut Display, grid: &mut Grid, r: usize, c: usize) -> usize {
    let mut n = 0;

    display.frame[r][c] = [255, 0, 0];
    display.emit_frame().unwrap();

    if r == 199 {
        display.frame[r][c] = [50, 50, 50];
        display.emit_frame().unwrap();
        return n;
    }

    if !grid[(r + 1, c)] {
        n += drop(display, grid, r + 1, c);

        if !grid[(r + 1, c)] {
            display.frame[r][c] = [50, 50, 50];
            display.emit_frame().unwrap();
            return n;
        }
    }

    if !grid[(r + 1, c - 1)] {
        n += drop(display, grid, r + 1, c - 1);

        if !grid[(r + 1, c - 1)] {
            display.frame[r][c] = [50, 50, 50];
            display.emit_frame().unwrap();
            return n;
        }
    }

    if !grid[(r + 1, c + 1)] {
        n += drop(display, grid, r + 1, c + 1);

        if !grid[(r + 1, c + 1)] {
            display.frame[r][c] = [50, 50, 50];
            display.emit_frame().unwrap();
            return n;
        }
    }

    // only fill in cell if all three cells below it are also filled
    grid.set((r, c), true);
    display.frame[r][c] = [0, 255, 0];
    display.emit_frame().unwrap();

    return n + 1;
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut display = Display::new();

    let mut grid = inner(&mut display, &stdin);

    println!("{}", drop(&mut display, &mut grid, 0, 500));
}

fn inner(display: &mut Display, stdin: &str) -> Grid {
    let mut grid = Grid {
        // cells: Box::new([false; HEIGHT * WIDTH]),
        cells: Box::new(BitArray::new([0u8; WIDTH * HEIGHT / 8])),
    };

    let lines: Vec<&str> = stdin.lines().collect();
    // lines.sort();
    // lines.dedup();

    for line in lines {
        let mut pairs = line
            .split(&['-', '>', ' ', ','])
            .filter(|i| !i.is_empty())
            .array_chunks()
            .map(|[m1, m2]| (m1.parse::<usize>().unwrap(), m2.parse::<usize>().unwrap()));

        // let mut pairs = regex.find_iter(line).array_chunks().map(|[m1, m2]| {
        //     (
        //         m1.as_str().parse::<usize>().unwrap(),
        //         m2.as_str().parse::<usize>().unwrap(),
        //     )
        // });

        let Some(mut last) = pairs.next() else { continue; };

        for next in pairs {
            // draw line from last to next
            let r_min = last.1.min(next.1);
            let r_max = last.1.max(next.1);
            let c_min = last.0.min(next.0);
            let c_max = last.0.max(next.0);
            for r in r_min..=r_max {
                for c in c_min..=c_max {
                    // grid[(r, c)] = true;
                    grid.set((r, c), true);
                    display.frame[r][c].fill(255);
                }
            }

            last = next;
        }
    }

    display.emit_frame().unwrap();

    grid
}

// drop(&mut grid, 0, 500)

// #[bench]
// fn bench(bencher: &mut Bencher) {
//     let stdin = std::include_str!("../../inputs/day14");

//     bencher.iter(|| test::black_box(drop(&mut inner(&stdin), 0, 500)))
// }

// #[bench]
// fn bench_1(bencher: &mut Bencher) {
//     let stdin = std::include_str!("../../inputs/day14");
//     let parsed = inner(&stdin);

//     bencher.iter(|| test::black_box(drop(&mut parsed.clone(), 0, 500)))
// }
