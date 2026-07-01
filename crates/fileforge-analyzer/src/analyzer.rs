use std::path::Path;

use fileforge_core::{AnalysisResult, Result};

pub trait Analyzer: Send + Sync {
    /// 表示名
    fn name(&self) -> &'static str;

    /// 対応拡張子
    fn extensions(&self) -> &'static [&'static str];

    /// 解析
    fn analyze(&self, path: &Path) -> Result<AnalysisResult>;
}
