use std::{fs::File, io::{BufRead, BufReader}, path::PathBuf};

use fileforge_core::Result;

#[derive(Debug, Clone)]
pub struct LogStats {
    pub total: u64,
    pub info: u64,
    pub warn: u64,
    pub error: u64,
    pub other: u64,
}

pub fn stats_log(path: PathBuf) -> Result<LogStats> {
    let file = File::open(&path)?;
    let reader = BufReader::with_capacity(8 * 1024 * 1024, file);

    let mut stats = LogStats { total: 0, info: 0, warn: 0, error: 0, other: 0 };

    for line in reader.lines() {
        let line = line?;
        stats.total += 1;

        if line.contains("ERROR") {
            stats.error += 1;
        } else if line.contains("WARN") {
            stats.warn += 1;
        } else if line.contains("INFO") {
            stats.info += 1;
        } else {
            stats.other += 1;
        }
    }

    Ok(stats)
}
