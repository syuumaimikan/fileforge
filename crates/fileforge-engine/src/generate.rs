use std::{fs::File, io::{BufWriter, Write}, path::PathBuf};

use fileforge_core::Result;

pub fn generate_log(path: PathBuf, lines: u64) -> Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::with_capacity(8 * 1024 * 1024, file);

    for i in 0..lines {
        let level = match i % 10 {
            0 => "ERROR",
            1 => "WARN",
            _ => "INFO",
        };

        writeln!(
            writer,
            "2026-07-01 12:{:02}:{:02} {} User={} Message number {}",
            (i / 60) % 60,
            i % 60,
            level,
            i % 1000,
            i
        )?;
    }

    writer.flush()?;
    Ok(())
}
