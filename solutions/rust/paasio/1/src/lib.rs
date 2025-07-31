use std::io::{Read, Result, Write};

pub struct ReadStats<R>{ reader: R, reads: usize, bytes: usize }

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats { reader: wrapped, reads: 0, bytes: 0 }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads += 1;
        match self.reader.read(buf) {
            Ok(b) => { self.bytes += b; Ok(b) },
            err => err
        }
    }
}

pub struct WriteStats<W>{ writer: W, writes: usize, bytes: usize }

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats { writer: wrapped, writes: 0, bytes: 0 }
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;
        match self.writer.write(buf) {
            Ok(b) => { self.bytes += b; Ok(b) },
            err => err
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
