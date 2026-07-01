use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

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

        // 1000行ごとに位置を保存
        if line_number % 1000 == 1 {
            writeln!(writer, "{},{}", line_number, offset)?;
        }

        offset += bytes_read as u64;
        line_number += 1;
    }

    writer.flush()?;

    println!("index created: {}", index_path.display());
    println!("total lines: {}", line_number - 1);

    Ok(())
}
