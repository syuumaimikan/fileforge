use std::{collections::VecDeque, fs::File, io::{BufRead, BufReader}, path::PathBuf};

use fileforge_core::Result;

#[derive(Debug, Clone)]
pub struct ContextOptions {
    pub path: PathBuf,
    pub keyword: String,
    pub before: usize,
    pub after: usize,
    pub limit: usize,
}

pub fn search_context<F>(options: ContextOptions, mut on_line: F) -> Result<usize>
where
    F: FnMut(String),
{
    let file = File::open(&options.path)?;
    let reader = BufReader::with_capacity(8 * 1024 * 1024, file);

    let mut prev_lines: VecDeque<(usize, String)> = VecDeque::new();
    let mut after_left = 0usize;
    let mut count = 0usize;

    for (i, line) in reader.lines().enumerate() {
        let line_number = i + 1;
        let line = line?;

        if line.contains(&options.keyword) {
            on_line(format!("----- match {} -----", count + 1));

            for (n, prev) in &prev_lines {
                on_line(format!("{}- {}", n, prev));
            }

            on_line(format!("{}> {}", line_number, line));

            after_left = options.after;
            count += 1;

            if options.limit > 0 && count >= options.limit {
                break;
            }
        } else if after_left > 0 {
            on_line(format!("{}+ {}", line_number, line));
            after_left -= 1;
        }

        prev_lines.push_back((line_number, line));

        if prev_lines.len() > options.before {
            prev_lines.pop_front();
        }
    }

    Ok(count)
}
