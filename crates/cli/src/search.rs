use aho_corasick::AhoCorasick;

pub struct SearchEngine {
    matcher: AhoCorasick,
}

impl SearchEngine {
    pub fn new(patterns: &[String]) -> Self {
        let matcher = AhoCorasick::new(patterns).unwrap();

        Self { matcher }
    }

    pub fn is_match(&self, line: &str) -> bool {
        self.matcher.is_match(line)
    }
}
