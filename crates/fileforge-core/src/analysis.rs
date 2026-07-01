use std::{collections::HashMap, path::PathBuf};

use crate::SearchHit;

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub format: String,
    pub file: FileInfo,
    pub metadata: HashMap<String, String>,
    pub hits: Vec<SearchHit>,
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub extension: String,
}