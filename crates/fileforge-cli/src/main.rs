use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use fileforge_engine::{
    context::{search_context, ContextOptions},
    extract::{extract_lines, ExtractOptions},
    generate::generate_log,
    jsonl::{JsonlEngine, JsonlSearchOptions},
    regex_search::{RegexSearchEngine, RegexSearchOptions},
    search::{SearchEngine, SearchOptions},
    stats::stats_log,
};

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
        /// comma separated keywords. example: ERROR,WARN,FATAL
        keywords: String,

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
    Context {
        path: PathBuf,
        keyword: String,

        #[arg(long, default_value_t = 2)]
        before: usize,

        #[arg(long, default_value_t = 2)]
        after: usize,

        #[arg(long, default_value_t = 100)]
        limit: usize,
    },
    Extract {
        path: PathBuf,
        keyword: String,

        #[arg(short, long)]
        output: PathBuf,

        #[arg(long, default_value_t = 0)]
        limit: usize,
    },
    Stats {
        path: PathBuf,
    },
    GenerateLog {
        path: PathBuf,

        #[arg(long, default_value_t = 1_000_000)]
        lines: u64,
    },
    Index {
        path: PathBuf,
    },
    Jump {
        path: PathBuf,

        #[arg(long)]
        line: u64,

        #[arg(long, default_value_t = 20)]
        show: usize,
    },
    InspectText {
        path: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Search {
            path,
            keywords,
            limit,
        } => {
            let keywords = keywords
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();

            let count = SearchEngine::search_text(
                SearchOptions {
                    path,
                    keywords,
                    limit,
                },
                |hit| {
                    println!("{}: {}", hit.line, hit.text);
                },
                |progress| {
                    eprint!(
                        "\rread: {}/{} bytes, matched: {}",
                        progress.read_bytes, progress.total_bytes, progress.matched
                    );
                },
            )?;

            eprintln!();
            eprintln!("matched: {}", count);
        }
        Commands::Regex {
            path,
            pattern,
            limit,
        } => {
            let count = RegexSearchEngine::search(
                RegexSearchOptions {
                    path,
                    pattern,
                    limit,
                },
                |hit| {
                    println!("{}: {}", hit.line, hit.text);
                },
                |progress| {
                    eprint!(
                        "\rread: {}/{} bytes, matched: {}",
                        progress.read_bytes, progress.total_bytes, progress.matched
                    );
                },
            )?;

            eprintln!();
            eprintln!("matched: {}", count);
        }
        Commands::Jsonl {
            path,
            key,
            value,
            limit,
        } => {
            let count = JsonlEngine::search(
                JsonlSearchOptions {
                    path,
                    key,
                    value,
                    limit,
                },
                |hit| {
                    println!("{}: {}", hit.line, hit.text);
                },
                |progress| {
                    eprint!(
                        "\rread: {}/{} bytes, matched: {}",
                        progress.read_bytes, progress.total_bytes, progress.matched
                    );
                },
            )?;

            eprintln!();
            eprintln!("matched: {}", count);
        }
        Commands::Context {
            path,
            keyword,
            before,
            after,
            limit,
        } => {
            let count = search_context(
                ContextOptions {
                    path,
                    keyword,
                    before,
                    after,
                    limit,
                },
                |line| println!("{}", line),
            )?;
            eprintln!("matched: {}", count);
        }
        Commands::Extract {
            path,
            keyword,
            output,
            limit,
        } => {
            let count = extract_lines(ExtractOptions {
                path,
                keyword,
                output: output.clone(),
                limit,
            })?;
            println!("extracted: {}", count);
            println!("output: {}", output.display());
        }
        Commands::Stats { path } => {
            let stats = stats_log(path)?;
            println!("total: {}", stats.total);
            println!("INFO : {}", stats.info);
            println!("WARN : {}", stats.warn);
            println!("ERROR: {}", stats.error);
            println!("OTHER: {}", stats.other);
        }
        Commands::GenerateLog { path, lines } => {
            generate_log(path, lines)?;
            println!("generated: {} lines", lines);
        }
        Commands::Index { path } => {
            fileforge_index::build_line_index(path.clone())?;
            println!("index created: {}", path.with_extension("ffidx").display());
        }
        Commands::Jump { path, line, show } => {
            fileforge_index::jump_line(path, line, show, |line| println!("{}", line))?;
        }
        Commands::InspectText { path } => {
            let info = fileforge_text::inspect_text(&path)?;
            println!("bytes: {}", info.bytes);
            println!("lines: {}", info.lines);
        }
    }

    Ok(())
}
