use std::{collections::HashMap, sync::Arc};

use crate::Analyzer;

pub struct AnalyzerManager {
    analyzers: HashMap<String, Arc<dyn Analyzer>>,
}

impl AnalyzerManager {
    pub fn new() -> Self {
        Self {
            analyzers: HashMap::new(),
        }
    }

    pub fn register<A>(&mut self, analyzer: A)
    where
        A: Analyzer + 'static,
    {
        let analyzer = Arc::new(analyzer);

        for ext in analyzer.extensions() {
            self.analyzers
                .insert(ext.to_ascii_lowercase(), analyzer.clone());
        }
    }

    pub fn get(&self, ext: &str) -> Option<Arc<dyn Analyzer>> {
        self.analyzers.get(&ext.to_ascii_lowercase()).cloned()
    }
}
