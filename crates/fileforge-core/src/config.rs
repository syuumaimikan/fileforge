#[derive(Debug, Clone)]
pub struct Config {
    pub buffer_size: usize,
    pub progress_update_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self { buffer_size: 8 * 1024 * 1024, progress_update_ms: 500 }
    }
}
