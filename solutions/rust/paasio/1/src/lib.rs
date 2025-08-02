use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    num_bytes: usize,
    num_reads: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {
            wrapped,
            num_bytes: 0,
            num_reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.num_bytes
    }

    pub fn reads(&self) -> usize {
        self.num_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self.wrapped.read(buf) {
            Ok(bytes) => {
                self.num_reads += 1;
                self.num_bytes += bytes;
                Ok(bytes)
            }
            Err(e) => Err(e),
        }
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    num_writes: usize,
    num_bytes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            wrapped,
            num_writes: 0,
            num_bytes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.num_bytes
    }

    pub fn writes(&self) -> usize {
        self.num_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self.wrapped.write(buf) {
            Ok(bytes) => {
                self.num_writes += 1;
                self.num_bytes += bytes;
                Ok(bytes)
            }
            Err(e) => Err(e),
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
