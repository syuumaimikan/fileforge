use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use fileforge_analyzer::AnalyzerManager;
use fileforge_core::Config;
use fileforge_csv::CsvAnalyzer;
use fileforge_engine::{
    context::{search_context, ContextOptions},
    extract::{extract_lines, ExtractOptions},
    generate::generate_log,
    jsonl::{JsonlEngine, JsonlSearchOptions},
    regex_search::{RegexSearchEngine, RegexSearchOptions},
    search::{SearchEngine, SearchOptions},
    stats::stats_log,
};
use fileforge_jsonl::JsonlAnalyzer;
use fileforge_query::{QueryEngine, QueryParser};
use fileforge_storage::ReaderFactory;
use fileforge_text::TextAnalyzer;

#[derive(Parser)]
#[command(name = "fileforge")]
#[command(version = "0.3.0")]
#[command(about = "Large file analyzer")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Search { path: PathBuf, keywords: String, #[arg(long, default_value_t = 100)] limit: usize },
    Regex { path: PathBuf, pattern: String, #[arg(long, default_value_t = 100)] limit: usize },
    Jsonl { path: PathBuf, key: String, value: String, #[arg(long, default_value_t = 100)] limit: usize },
    Context { path: PathBuf, keyword: String, #[arg(long, default_value_t = 2)] before: usize, #[arg(long, default_value_t = 2)] after: usize, #[arg(long, default_value_t = 100)] limit: usize },
    Extract { path: PathBuf, keyword: String, #[arg(short, long)] output: PathBuf, #[arg(long, default_value_t = 0)] limit: usize },
    Stats { path: PathBuf },
    GenerateLog { path: PathBuf, #[arg(long, default_value_t = 1_000_000)] lines: u64 },
    Index { path: PathBuf },
    Jump { path: PathBuf, #[arg(long)] line: u64, #[arg(long, default_value_t = 20)] show: usize },
    InspectText { path: PathBuf },
    Analyze { path: PathBuf },
    Query { path: PathBuf, expr: String, #[arg(long, default_value_t = 100)] limit: usize },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Search { path, keywords, limit } => {
            let keywords = keywords.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect::<Vec<_>>();
            let count = SearchEngine::search_text(
                SearchOptions { path, keywords, limit },
                |hit| println!("{}: {}", hit.line, hit.text),
                |progress| eprint!("\rread: {}/{} bytes, matched: {}", progress.read_bytes, progress.total_bytes, progress.matched),
            )?;
            eprintln!();
            eprintln!("matched: {}", count);
        }
        Commands::Regex { path, pattern, limit } => {
            let count = RegexSearchEngine::search(
                RegexSearchOptions { path, pattern, limit },
                |hit| println!("{}: {}", hit.line, hit.text),
                |progress| eprint!("\rread: {}/{} bytes, matched: {}", progress.read_bytes, progress.total_bytes, progress.matched),
            )?;
            eprintln!();
            eprintln!("matched: {}", count);
        }
        Commands::Jsonl { path, key, value, limit } => {
            let count = JsonlEngine::search(
                JsonlSearchOptions { path, key, value, limit },
                |hit| println!("{}: {}", hit.line, hit.text),
                |progress| eprint!("\rread: {}/{} bytes, matched: {}", progress.read_bytes, progress.total_bytes, progress.matched),
            )?;
            eprintln!();
            eprintln!("matched: {}", count);
        }
        Commands::Context { path, keyword, before, after, limit } => {
            let count = search_context(ContextOptions { path, keyword, before, after, limit }, |line| println!("{}", line))?;
            eprintln!("matched: {}", count);
        }
        Commands::Extract { path, keyword, output, limit } => {
            let count = extract_lines(ExtractOptions { path, keyword, output: output.clone(), limit })?;
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
        Commands::Analyze { path } => {
            let mut manager = AnalyzerManager::new();
            manager.register(TextAnalyzer);
            manager.register(CsvAnalyzer);
            manager.register(JsonlAnalyzer);
            let result = manager.open(&path)?;
            println!("format   : {}", result.format);
            println!("path     : {}", result.file.path.display());
            println!("size     : {} bytes", result.file.size);
            println!("extension: {}", result.file.extension);
            println!("lines    : {}", result.statistics.lines);
            println!("objects  : {}", result.statistics.objects);
            println!("metadata:");
            for (key, value) in result.metadata { println!("  {}: {}", key, value); }
        }
        Commands::Query { path, expr, limit } => {
            let config = Config::default();
            let mut reader = ReaderFactory::open(&path, &config)?;
            let query = QueryParser::parse(&expr, limit)?;
            let count = QueryEngine::run(reader.as_mut(), query, |record| {
                println!("{}: {}", record.line, record.data);
            })?;
            eprintln!("matched: {}", count);
        }
    }
    Ok(())
}
