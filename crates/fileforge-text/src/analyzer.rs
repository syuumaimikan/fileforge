use std::{collections::HashMap, path::Path};

use fileforge_analyzer::{Analyzer, FileProbe};
use fileforge_core::{AnalysisResult, Config, FileInfo, Result, Statistics};
use fileforge_encoding::detect_encoding;
use fileforge_storage::ReaderFactory;

pub struct TextAnalyzer;

impl Analyzer for TextAnalyzer {
    fn name(&self) -> &'static str { "Text" }
    fn can_open(&self, probe: &FileProbe) -> bool { matches!(probe.extension.as_deref(), Some("txt") | Some("log")) }
    fn analyze(&self, path: &Path) -> Result<AnalysisResult> {
        let probe = FileProbe::from_path(path)?;
        let encoding = detect_encoding(&probe.header);
        let mut metadata = HashMap::new();
        metadata.insert("encoding".to_string(), format!("{:?}", encoding));
        let config = Config::default();
        let mut reader = ReaderFactory::open(path, &config)?;
        let mut line_count = 0u64;
        while let Some(_record) = reader.next_record()? { line_count += 1; }
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_string();
        Ok(AnalysisResult {
            format: "text".to_string(),
            file: FileInfo { path: path.to_path_buf(), size: probe.size, extension },
            metadata,
            statistics: Statistics { lines: line_count, size: probe.size, objects: line_count },
        })
    }
}
