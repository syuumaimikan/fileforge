use std::{
    borrow::Cow,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use fileforge_core::Result;

use crate::{Reader, Record};

pub struct LineReader {
    reader: BufReader<File>,
    line: u64,
    offset: u64,
}

impl LineReader {
    pub fn open(path: &Path, buffer_size: usize) -> Result<Self> {
        let file = File::open(path)?;
        Ok(Self { reader: BufReader::with_capacity(buffer_size, file), line: 0, offset: 0 })
    }
}

impl Reader for LineReader {
    fn next_record(&mut self) -> Result<Option<Record<'_>>> {
        let mut buf = String::new();
        let read = self.reader.read_line(&mut buf)?;
        if read == 0 { return Ok(None); }
        self.line += 1;
        let text = buf.trim_end_matches(['\r', '\n']).to_string();
        let record = Record { line: self.line, offset: self.offset, data: Cow::Owned(text) };
        self.offset += read as u64;
        Ok(Some(record))
    }
}
