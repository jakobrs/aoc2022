use std::io::BufRead;

use crate::LendingIterator;

pub struct LinesBuf<Reader: BufRead> {
    buf: String,
    reader: Reader,
}

impl<Reader: BufRead> LendingIterator for LinesBuf<Reader> {
    type Item<'a> = std::io::Result<&'a str> where Reader: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        // Adapted from stdlib implementation of Lines
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

pub trait BufReadExt: BufRead {
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
