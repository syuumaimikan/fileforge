use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SearchHit {
    pub file: PathBuf,
    pub line: u64,
    pub offset: u64,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct SearchStats {
    pub total_lines: u64,
    pub matched_lines: u64,
    pub elapsed_ms: u128,
}
