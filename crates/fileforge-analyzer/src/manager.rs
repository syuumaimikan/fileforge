use std::{path::Path, sync::Arc};

use fileforge_core::{AnalysisResult, Result};

use crate::{Analyzer, FileProbe};

pub struct AnalyzerManager { analyzers: Vec<Arc<dyn Analyzer>> }

impl AnalyzerManager {
    pub fn new() -> Self { Self { analyzers: Vec::new() } }

    pub fn register<A>(&mut self, analyzer: A) where A: Analyzer + 'static {
        self.analyzers.push(Arc::new(analyzer));
    }

    pub fn open<P: AsRef<Path>>(&self, path: P) -> Result<AnalysisResult> {
        let path = path.as_ref();
        let probe = FileProbe::from_path(path)?;
        for analyzer in &self.analyzers {
            if analyzer.can_open(&probe) { return analyzer.analyze(path); }
        }
        anyhow::bail!("対応するAnalyzerが見つかりません: {}", path.display());
    }
}

impl Default for AnalyzerManager { fn default() -> Self { Self::new() } }
