use std::{collections::HashMap, path::PathBuf};

use crate::SearchHit;

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub format: String,

    pub file: FileInfo,

    pub metadata: HashMap<String, String>,

    pub statistics: Statistics,

    pub objects: Vec<Object>,
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

#[derive(Debug, Clone)]
pub enum Object {
    Text(TextObject),

    Csv(CsvObject),

    Json(JsonObject),

    Binary(BinaryObject),
}

#[derive(Debug, Clone)]
pub struct TextObject {
    pub line: u64,

    pub offset: u64,

    pub text: String,
}

#[derive(Debug, Clone)]
pub struct CsvObject {
    pub row: u64,

    pub values: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct JsonObject {
    pub path: String,

    pub value: String,
}

#[derive(Debug, Clone)]
pub struct BinaryObject {
    pub name: String,

    pub offset: u64,

    pub size: u64,
}
