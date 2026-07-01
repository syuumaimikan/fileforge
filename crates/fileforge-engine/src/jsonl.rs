use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    time::Instant,
};

use fileforge_core::{Result, SearchHit};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct JsonlSearchOptions {
    pub path: PathBuf,
    pub key: String,
    pub value: String,
    pub limit: usize,
}

#[derive(Debug, Clone)]
pub struct JsonlProgress {
    pub read_bytes: u64,
    pub total_bytes: u64,
    pub matched: usize,
}

pub struct JsonlEngine;

impl JsonlEngine {
    pub fn search<F, P>(
        options: JsonlSearchOptions,
        mut on_hit: F,
        mut on_progress: P,
    ) -> Result<usize>
    where
        F: FnMut(SearchHit),
        P: FnMut(JsonlProgress),
    {
        let file = File::open(&options.path)?;
        let total_bytes = file.metadata()?.len();
        let reader = BufReader::with_capacity(8 * 1024 * 1024, file);

        let mut offset = 0u64;
        let mut count = 0usize;
        let mut last_progress = Instant::now();

        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            let line_size = line.len() as u64 + 1;

            let matched = match serde_json::from_str::<Value>(&line) {
                Ok(json) => json
                    .get(&options.key)
                    .map(|v| match v {
                        Value::String(s) => s == &options.value,
                        Value::Number(n) => n.to_string() == options.value,
                        Value::Bool(b) => b.to_string() == options.value,
                        _ => false,
                    })
                    .unwrap_or(false),
                Err(_) => false,
            };

            if matched {
                on_hit(SearchHit {
                    file: options.path.clone(),
                    line: i as u64 + 1,
                    offset,
                    text: line.clone(),
                });
                count += 1;

                if options.limit > 0 && count >= options.limit {
                    break;
                }
            }

            offset += line_size;

            if last_progress.elapsed().as_millis() >= 500 {
                on_progress(JsonlProgress {
                    read_bytes: offset,
                    total_bytes,
                    matched: count,
                });
                last_progress = Instant::now();
            }
        }

        on_progress(JsonlProgress {
            read_bytes: offset,
            total_bytes,
            matched: count,
        });

        Ok(count)
    }
}
