use std::{collections::{BTreeSet, HashMap}, path::Path};

use fileforge_analyzer::{Analyzer, FileProbe};
use fileforge_core::{AnalysisResult, Config, FileInfo, Result, Statistics};
use fileforge_encoding::detect_encoding;
use fileforge_storage::ReaderFactory;
use serde_json::Value;

pub struct JsonlAnalyzer;

impl Analyzer for JsonlAnalyzer {
    fn name(&self) -> &'static str { "JSONL" }

    fn can_open(&self, probe: &FileProbe) -> bool {
        matches!(probe.extension.as_deref(), Some("jsonl") | Some("ndjson"))
    }

    fn analyze(&self, path: &Path) -> Result<AnalysisResult> {
        let probe = FileProbe::from_path(path)?;
        let encoding = detect_encoding(&probe.header);
        let config = Config::default();
        let mut reader = ReaderFactory::open(path, &config)?;
        let mut rows = 0u64;
        let mut valid_json = 0u64;
        let mut invalid_json = 0u64;
        let mut keys = BTreeSet::new();

        while let Some(record) = reader.next_record()? {
            let line = record.data.trim();
            if line.is_empty() { continue; }
            rows += 1;
            match serde_json::from_str::<Value>(line) {
                Ok(Value::Object(map)) => {
                    valid_json += 1;
                    for key in map.keys() { keys.insert(key.clone()); }
                }
                Ok(_) => valid_json += 1,
                Err(_) => invalid_json += 1,
            }
        }

        let mut metadata = HashMap::new();
        metadata.insert("encoding".to_string(), format!("{:?}", encoding));
        metadata.insert("valid_json".to_string(), valid_json.to_string());
        metadata.insert("invalid_json".to_string(), invalid_json.to_string());
        metadata.insert("keys".to_string(), keys.into_iter().collect::<Vec<_>>().join(", "));
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_string();

        Ok(AnalysisResult {
            format: "jsonl".to_string(),
            file: FileInfo { path: path.to_path_buf(), size: probe.size, extension },
            metadata,
            statistics: Statistics { lines: rows, size: probe.size, objects: valid_json },
        })
    }
}
