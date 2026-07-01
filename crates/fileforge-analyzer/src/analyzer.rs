use std::path::Path;

use fileforge_core::{AnalysisResult, Result};

use crate::FileProbe;

pub trait Analyzer: Send + Sync {
    fn name(&self) -> &'static str;
    fn can_open(&self, probe: &FileProbe) -> bool;
    fn analyze(&self, path: &Path) -> Result<AnalysisResult>;
}
