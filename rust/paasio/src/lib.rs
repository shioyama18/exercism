use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    reader: R,
    nbytes: usize,
    num_reads: usize,
}

impl<R: Read> ReadStats<R> {

    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            reader: wrapped,
            nbytes: 0,
            num_reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.nbytes
    }

    pub fn reads(&self) -> usize {
        self.num_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let read = self.reader.read(buf)?;
        self.nbytes += read;
        self.num_reads += 1;
        Ok(read)
    }
}

pub struct WriteStats<W> {
    writer: W,
    nbytes: usize,
    num_writes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            writer: wrapped,
            nbytes: 0,
            num_writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.nbytes
    }

    pub fn writes(&self) -> usize {
        self.num_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let written = self.writer.write(buf)?;
        self.nbytes += written;
        self.num_writes += 1;
        Ok(written)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()?;
        Ok(())
    }
}
