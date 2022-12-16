use std::io::BufRead;

trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}

struct LinesBuf<Reader: BufRead> {
    buf: String,
    reader: Reader,
}

impl<Reader: BufRead> LendingIterator for LinesBuf<Reader> {
    type Item<'a> = std::io::Result<&'a str> where Reader: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.buf.clear();
        match self.reader.read_line(&mut self.buf) {
            Ok(0) => None,
            Ok(_n) => {
                if self.buf.ends_with('\n') {
                    self.buf.pop();
                    if self.buf.ends_with('\r') {
                        self.buf.pop();
                    }
                }
                Some(Ok(&self.buf))
            }
            Err(e) => Some(Err(e)),
        }
    }
}

trait BufReadExt: BufRead {
    fn lines_buf(self) -> LinesBuf<Self>
    where
        Self: Sized;
}

impl<T: BufRead> BufReadExt for T {
    fn lines_buf(self) -> LinesBuf<Self>
    where
        Self: Sized,
    {
        LinesBuf {
            buf: String::new(),
            reader: self,
        }
    }
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
        let to = line[2] - b'X';

        let result = (to + 3 - from + 1) % 3;

        total += to as u64 + 1 + result as u64 * 3;
    }

    println!("{total}");

    Ok(())
}
