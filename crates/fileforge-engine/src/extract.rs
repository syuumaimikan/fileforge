use std::{fs::File, io::{BufRead, BufReader, BufWriter, Write}, path::PathBuf};

use fileforge_core::Result;

#[derive(Debug, Clone)]
pub struct ExtractOptions {
    pub path: PathBuf,
    pub keyword: String,
    pub output: PathBuf,
    pub limit: usize,
}

pub fn extract_lines(options: ExtractOptions) -> Result<usize> {
    let input = File::open(&options.path)?;
    let reader = BufReader::with_capacity(8 * 1024 * 1024, input);

    let output_file = File::create(&options.output)?;
    let mut writer = BufWriter::with_capacity(8 * 1024 * 1024, output_file);

    let mut count = 0usize;

    for line in reader.lines() {
        let line = line?;

        if line.contains(&options.keyword) {
            writeln!(writer, "{}", line)?;
            count += 1;

            if options.limit > 0 && count >= options.limit {
                break;
            }
        }
    }

    writer.flush()?;
    Ok(count)
}
