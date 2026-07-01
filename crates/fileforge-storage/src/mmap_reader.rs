use std::{borrow::Cow, fs::File, path::Path, str};

use fileforge_core::Result;
use memmap2::Mmap;

use crate::{Reader, Record};

pub struct MemoryMapReader {
    mmap: Mmap,
    cursor: usize,
    line: u64,
}

impl MemoryMapReader {
    pub fn open(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        Ok(Self { mmap, cursor: 0, line: 0 })
    }
}

impl Reader for MemoryMapReader {
    fn next_record(&mut self) -> Result<Option<Record<'_>>> {
        if self.cursor >= self.mmap.len() { return Ok(None); }
        let start = self.cursor;
        while self.cursor < self.mmap.len() && self.mmap[self.cursor] != b'\n' { self.cursor += 1; }
        let end = self.cursor;
        if self.cursor < self.mmap.len() && self.mmap[self.cursor] == b'\n' { self.cursor += 1; }
        let mut slice = &self.mmap[start..end];
        if slice.ends_with(b"\r") { slice = &slice[..slice.len() - 1]; }
        let text = str::from_utf8(slice)?.to_string();
        self.line += 1;
        Ok(Some(Record { line: self.line, offset: start as u64, data: Cow::Owned(text) }))
    }
}
