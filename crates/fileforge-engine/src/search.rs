use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    time::Instant,
};

use aho_corasick::AhoCorasick;
use fileforge_core::{Result, SearchHit};

#[derive(Debug, Clone)]
pub struct SearchOptions {
    pub path: PathBuf,
    pub keywords: Vec<String>,
    pub limit: usize,
}

#[derive(Debug, Clone)]
pub struct SearchProgress {
    pub read_bytes: u64,
    pub total_bytes: u64,
    pub matched: usize,
}

pub struct SearchEngine;

impl SearchEngine {
    pub fn search_text<F, P>(
        options: SearchOptions,
        mut on_hit: F,
        mut on_progress: P,
    ) -> Result<usize>
    where
        F: FnMut(SearchHit),
        P: FnMut(SearchProgress),
    {
        let matcher = AhoCorasick::new(&options.keywords)?;
        let file = File::open(&options.path)?;
        let total_bytes = file.metadata()?.len();
        let reader = BufReader::with_capacity(8 * 1024 * 1024, file);

        let mut offset = 0u64;
        let mut count = 0usize;
        let mut last_progress = Instant::now();

        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            let line_size = line.len() as u64 + 1;

            if matcher.is_match(&line) {
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
                on_progress(SearchProgress {
                    read_bytes: offset,
                    total_bytes,
                    matched: count,
                });
                last_progress = Instant::now();
            }
        }

        on_progress(SearchProgress {
            read_bytes: offset,
            total_bytes,
            matched: count,
        });

        Ok(count)
    }
}
