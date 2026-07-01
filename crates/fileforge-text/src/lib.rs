use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use fileforge_core::Result;

#[derive(Debug, Clone)]
pub struct TextInfo {
    pub lines: u64,
    pub bytes: u64,
}

pub fn inspect_text(path: &Path) -> Result<TextInfo> {
    let file = File::open(path)?;
    let bytes = file.metadata()?.len();
    let reader = BufReader::with_capacity(8 * 1024 * 1024, file);
    let mut lines = 0u64;

    for line in reader.lines() {
        line?;
        lines += 1;
    }

    Ok(TextInfo { lines, bytes })
}
