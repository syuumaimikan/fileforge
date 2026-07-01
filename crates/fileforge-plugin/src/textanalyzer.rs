impl Analyzer for TextAnalyzer {
    fn name(&self) -> &'static str {
        "Text"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["txt", "log"]
    }

    fn analyze(&self, path: &Path) -> Result<()> {
        println!("{}", path.display());

        Ok(())
    }
}
