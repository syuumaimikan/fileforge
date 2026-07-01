use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write},
    path::PathBuf,
};

use fileforge_core::Result;

pub fn build_line_index(path: PathBuf) -> Result<()> {
    let input = File::open(&path)?;
    let mut reader = BufReader::with_capacity(8 * 1024 * 1024, input);

    let index_path = path.with_extension("ffidx");
    let output = File::create(&index_path)?;
    let mut writer = BufWriter::new(output);

    let mut offset: u64 = 0;
    let mut line_number: u64 = 1;
    let mut buffer = Vec::new();

    writeln!(writer, "FFIDX_V1")?;

    loop {
        buffer.clear();
        let bytes_read = reader.read_until(b'\n', &mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        if line_number % 1000 == 1 {
            writeln!(writer, "{},{}", line_number, offset)?;
        }

        offset += bytes_read as u64;
        line_number += 1;
    }

    writer.flush()?;
    Ok(())
}

pub fn jump_line<F>(path: PathBuf, target_line: u64, show: usize, mut on_line: F) -> Result<()>
where
    F: FnMut(String),
{
    let index_path = path.with_extension("ffidx");
    let index_file = File::open(&index_path)?;
    let index_reader = BufReader::new(index_file);

    let mut nearest_line = 1u64;
    let mut nearest_offset = 0u64;

    for line in index_reader.lines().skip(1) {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() != 2 {
            continue;
        }

        let line_number: u64 = parts[0].parse()?;
        let offset: u64 = parts[1].parse()?;

        if line_number <= target_line {
            nearest_line = line_number;
            nearest_offset = offset;
        } else {
            break;
        }
    }

    let mut file = File::open(&path)?;
    file.seek(SeekFrom::Start(nearest_offset))?;

    let reader = BufReader::with_capacity(8 * 1024 * 1024, file);

    for (i, line) in reader.lines().enumerate() {
        let current_line = nearest_line + i as u64;

        if current_line < target_line {
            continue;
        }

        if current_line >= target_line + show as u64 {
            break;
        }

        on_line(format!("{}: {}", current_line, line?));
    }

    Ok(())
}
