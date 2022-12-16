use anyhow::Result;
use aoc2022::{BufReadExt, LendingIterator};

fn main() -> Result<()> {
    let stdin = std::io::stdin().lock();

    let mut total: u64 = 0;

    let mut it = stdin.lines_buf();
    while let Some(line) = it.next() {
        let line = line?.as_bytes();

        if line.len() == 0 {
            break;
        }

        let from = line[0] - b'A';
        let result = line[2] - b'X';

        let to = (from + result + 3 - 1) % 3;

        total += to as u64 + 1 + result as u64 * 3;
    }

    println!("{total}");

    Ok(())
}
