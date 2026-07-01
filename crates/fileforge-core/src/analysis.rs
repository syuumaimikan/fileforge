use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub format: String,
    pub file: FileInfo,
    pub metadata: HashMap<String, String>,
    pub statistics: Statistics,
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub extension: String,
}

#[derive(Debug, Clone)]
pub struct Statistics {
    pub lines: u64,
    pub size: u64,
    pub objects: u64,
}
