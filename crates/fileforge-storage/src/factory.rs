use std::path::Path;

use fileforge_core::{Config, Result};

use crate::{ChunkReader, LineReader, MemoryMapReader, Reader};

pub enum ReaderKind { Line, Chunk, MemoryMap }

pub struct ReaderFactory;

impl ReaderFactory {
    pub fn open(path: &Path, config: &Config) -> Result<Box<dyn Reader>> {
        let size = std::fs::metadata(path)?.len();
        match Self::select_reader(size) {
            ReaderKind::Line => Ok(Box::new(LineReader::open(path, config.buffer_size)?)),
            ReaderKind::Chunk => Ok(Box::new(ChunkReader::open(path, config.buffer_size)?)),
            ReaderKind::MemoryMap => Ok(Box::new(MemoryMapReader::open(path)?)),
        }
    }

    fn select_reader(size: u64) -> ReaderKind {
        if size < 100 * 1024 * 1024 { ReaderKind::Line }
        else if size < 10 * 1024 * 1024 * 1024 { ReaderKind::Chunk }
        else { ReaderKind::MemoryMap }
    }
}
