use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    time::Instant,
};

use fileforge_core::{Result, SearchHit};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct RegexSearchOptions {
    pub path: PathBuf,
    pub pattern: String,
    pub limit: usize,
}

#[derive(Debug, Clone)]
pub struct RegexSearchProgress {
    pub read_bytes: u64,
    pub total_bytes: u64,
    pub matched: usize,
}

pub struct RegexSearchEngine;

impl RegexSearchEngine {
    pub fn search<F, P>(
        options: RegexSearchOptions,
        mut on_hit: F,
        mut on_progress: P,
    ) -> Result<usize>
    where
        F: FnMut(SearchHit),
        P: FnMut(RegexSearchProgress),
    {
        let re = Regex::new(&options.pattern)?;
        let file = File::open(&options.path)?;
        let total_bytes = file.metadata()?.len();
        let reader = BufReader::with_capacity(8 * 1024 * 1024, file);

        let mut offset = 0u64;
        let mut count = 0usize;
        let mut last_progress = Instant::now();

        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            let line_size = line.len() as u64 + 1;

            if re.is_match(&line) {
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
                on_progress(RegexSearchProgress {
                    read_bytes: offset,
                    total_bytes,
                    matched: count,
                });
                last_progress = Instant::now();
            }
        }

        on_progress(RegexSearchProgress {
            read_bytes: offset,
            total_bytes,
            matched: count,
        });

        Ok(count)
    }
}
