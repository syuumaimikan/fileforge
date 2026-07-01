use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use anyhow::Result;
use clap::{Parser, Subcommand};
use regex::Regex;
use serde_json::Value;

use indicatif::{ProgressBar, ProgressStyle};
use std::io::{Seek, SeekFrom};

use std::io::{BufWriter, Write};

fn generate_log(path: PathBuf, lines: u64) -> Result<()> {
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

    println!("Generated {} lines.", lines);

    Ok(())
}

#[derive(Parser)]
#[command(name = "fileforge")]
#[command(version = "0.1.0")]
#[command(about = "Large file analyzer")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Search {
        path: PathBuf,
        keyword: String,

        #[arg(long, default_value_t = 100)]
        limit: usize,
    },

    Regex {
        path: PathBuf,
        pattern: String,

        #[arg(long, default_value_t = 100)]
        limit: usize,
    },

    Jsonl {
        path: PathBuf,
        key: String,
        value: String,

        #[arg(long, default_value_t = 100)]
        limit: usize,
    },

    Csv {
        path: PathBuf,
        column: String,
        keyword: String,

        #[arg(long, default_value_t = 100)]
        limit: usize,
    },
    GenerateLog {
        path: PathBuf,

        #[arg(long, default_value_t = 1_000_000)]
        lines: u64,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Search {
            path,
            keyword,
            limit,
        } => search_text(path, keyword, limit)?,

        Commands::Regex {
            path,
            pattern,
            limit,
        } => search_regex(path, pattern, limit)?,

        Commands::Jsonl {
            path,
            key,
            value,
            limit,
        } => search_jsonl(path, key, value, limit)?,

        Commands::Csv {
            path,
            column,
            keyword,
            limit,
        } => search_csv(path, column, keyword, limit)?,

        Commands::GenerateLog { path, lines } => generate_log(path, lines)?,
    }

    Ok(())
}

fn search_text(path: PathBuf, keyword: String, limit: usize) -> Result<()> {
    let mut file = File::open(&path)?;
    let file_size = file.seek(SeekFrom::End(0))?;
    file.seek(SeekFrom::Start(0))?;

    let pb = ProgressBar::new(file_size);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} {bytes_per_sec} ETA:{eta}"
        )?
        .progress_chars("#>-"),
    );

    let reader = BufReader::with_capacity(8 * 1024 * 1024, file);

    let mut count = 0usize;
    let mut read_bytes = 0u64;

    for (i, line) in reader.lines().enumerate() {
        let line = line?;

        read_bytes += line.len() as u64 + 1;
        pb.set_position(read_bytes.min(file_size));

        if line.contains(&keyword) {
            println!("{}: {}", i + 1, line);
            count += 1;

            if count >= limit {
                break;
            }
        }
    }

    pb.finish_with_message("done");
    eprintln!("matched: {}", count);

    Ok(())
}

fn search_regex(path: PathBuf, pattern: String, limit: usize) -> Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::with_capacity(8 * 1024 * 1024, file);
    let re = Regex::new(&pattern)?;

    let mut count = 0usize;

    for (i, line) in reader.lines().enumerate() {
        let line = line?;

        if re.is_match(&line) {
            println!("{}: {}", i + 1, line);
            count += 1;

            if count >= limit {
                break;
            }
        }
    }

    eprintln!("matched: {}", count);
    Ok(())
}

fn search_jsonl(path: PathBuf, key: String, value: String, limit: usize) -> Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::with_capacity(8 * 1024 * 1024, file);

    let mut count = 0usize;

    for (i, line) in reader.lines().enumerate() {
        let line = line?;

        let json: Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => continue,
        };

        if json.get(&key).and_then(|v| v.as_str()) == Some(value.as_str()) {
            println!("{}: {}", i + 1, line);
            count += 1;

            if count >= limit {
                break;
            }
        }
    }

    eprintln!("matched: {}", count);
    Ok(())
}

fn search_csv(path: PathBuf, column: String, keyword: String, limit: usize) -> Result<()> {
    let mut reader = csv::Reader::from_path(path)?;
    let headers = reader.headers()?.clone();

    let column_index = headers
        .iter()
        .position(|h| h == column)
        .ok_or_else(|| anyhow::anyhow!("column not found: {}", column))?;

    let mut count = 0usize;

    for result in reader.records() {
        let record = result?;

        if record.get(column_index).unwrap_or("").contains(&keyword) {
            println!("{}", record.iter().collect::<Vec<_>>().join(","));
            count += 1;

            if count >= limit {
                break;
            }
        }
    }

    eprintln!("matched: {}", count);
    Ok(())
}
