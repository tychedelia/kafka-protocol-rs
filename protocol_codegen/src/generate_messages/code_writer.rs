use std::io::{self, Write};

use failure::Error;

pub struct CodeWriter<W> {
    inner: W,
    indent: usize,
    new_line: bool,
}

impl<W: Write> CodeWriter<W> {
    pub fn new(inner: W) -> Self {
        Self {
            inner,
            indent: 0,
            new_line: true,
        }
    }
    pub fn indent<R, F: FnOnce(&mut Self) -> R>(&mut self, f: F) -> R {
        self.indent += 1;
        let res = f(self);
        self.indent -= 1;
        res
    }
    pub fn block<T, F: FnOnce(&mut Self) -> Result<T, Error>>(&mut self, f: F) -> Result<T, Error> {
        writeln!(self, "{{")?;
        let res = self.indent(f);
        if !self.new_line {
            writeln!(self)?;
        }
        write!(self, "}}")?;
        res
    }
    #[allow(dead_code)]
    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl<W: Write> Write for CodeWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut start_byte = 0;
        for (end_byte, &b) in buf.iter().enumerate() {
            if b == b'\n' {
                self.new_line = true;
            } else if self.new_line {
                self.new_line = false;

                self.inner.write_all(&buf[start_byte..end_byte])?;
                start_byte = end_byte;

                for _ in 0..self.indent {
                    self.inner.write_all(b"    ")?;
                }
            }
        }
        if start_byte < buf.len() {
            self.inner.write_all(&buf[start_byte..])?;
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
