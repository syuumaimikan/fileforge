use std::{path::PathBuf, time::Instant};

use aho_corasick::AhoCorasick;
use fileforge_core::{Config, Result, SearchHit};
use fileforge_storage::ReaderFactory;

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
    pub fn search_text<F, P>(options: SearchOptions, mut on_hit: F, mut on_progress: P) -> Result<usize>
    where
        F: FnMut(SearchHit),
        P: FnMut(SearchProgress),
    {
        let total_bytes = std::fs::metadata(&options.path)?.len();
        let matcher = AhoCorasick::new(&options.keywords)?;
        let config = Config::default();
        let mut reader = ReaderFactory::open(&options.path, &config)?;

        let mut count = 0usize;
        let mut last_progress = Instant::now();
        let mut read_bytes = 0u64;

        while let Some(record) = reader.next_record()? {
            read_bytes = record.offset + record.data.len() as u64 + 1;
            if matcher.is_match(record.data.as_ref()) {
                on_hit(SearchHit {
                    file: options.path.clone(),
                    line: record.line,
                    offset: record.offset,
                    text: record.data.to_string(),
                });
                count += 1;
                if options.limit > 0 && count >= options.limit { break; }
            }
            if last_progress.elapsed().as_millis() >= 500 {
                on_progress(SearchProgress { read_bytes, total_bytes, matched: count });
                last_progress = Instant::now();
            }
        }
        on_progress(SearchProgress { read_bytes, total_bytes, matched: count });
        Ok(count)
    }
}
