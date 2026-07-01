pub struct TextAnalyzer;

use std::{collections::HashMap, path::Path};

use fileforge_analyzer::{Analyzer, FileProbe};

use fileforge_core::{AnalysisResult, FileInfo, Result, Statistics};

pub struct TextAnalyzer;

fn can_open(&self, probe: &FileProbe) -> bool {
    matches!(probe.extension.as_deref(), Some("txt") | Some("log"))
}
