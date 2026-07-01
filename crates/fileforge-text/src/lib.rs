pub mod analyzer;

pub use analyzer::TextAnalyzer;

use std::{fs::File, io::{BufRead, BufReader}, path::Path};
use fileforge_core::Result;

pub struct TextInfo { pub bytes: u64, pub lines: u64 }

pub fn inspect_text(path: &Path) -> Result<TextInfo> {
    let file = File::open(path)?;
    let bytes = file.metadata()?.len();
    let reader = BufReader::new(file);
    let mut lines = 0u64;
    for line in reader.lines() { line?; lines += 1; }
    Ok(TextInfo { bytes, lines })
}
