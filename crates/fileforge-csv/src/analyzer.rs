use std::{collections::HashMap, path::Path};

use fileforge_analyzer::{Analyzer, FileProbe};
use fileforge_core::{AnalysisResult, FileInfo, Result, Statistics};
use fileforge_encoding::detect_encoding;

pub struct CsvAnalyzer;

impl Analyzer for CsvAnalyzer {
    fn name(&self) -> &'static str { "CSV" }

    fn can_open(&self, probe: &FileProbe) -> bool {
        matches!(probe.extension.as_deref(), Some("csv"))
    }

    fn analyze(&self, path: &Path) -> Result<AnalysisResult> {
        let probe = FileProbe::from_path(path)?;
        let encoding = detect_encoding(&probe.header);
        let mut reader = csv::Reader::from_path(path)?;
        let headers = reader.headers()?.clone();
        let mut rows = 0u64;
        for result in reader.records() { result?; rows += 1; }

        let mut metadata = HashMap::new();
        metadata.insert("encoding".to_string(), format!("{:?}", encoding));
        metadata.insert("columns".to_string(), headers.len().to_string());
        metadata.insert("headers".to_string(), headers.iter().collect::<Vec<_>>().join(", "));
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_string();

        Ok(AnalysisResult {
            format: "csv".to_string(),
            file: FileInfo { path: path.to_path_buf(), size: probe.size, extension },
            metadata,
            statistics: Statistics { lines: rows + 1, size: probe.size, objects: rows },
        })
    }
}
