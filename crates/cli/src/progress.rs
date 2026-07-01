use indicatif::{ProgressBar, ProgressStyle};
use std::time::{Duration, Instant};

pub struct Progress {
    pb: ProgressBar,
    last_update: Instant,
    bytes: u64,
}

impl Progress {
    pub fn new(total: u64) -> Self {
        let pb = ProgressBar::new(total);

        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] \
                 [{wide_bar:.cyan/blue}] \
                 {bytes}/{total_bytes} \
                 {bytes_per_sec} ETA:{eta}",
            )
            .unwrap(),
        );

        Self {
            pb,
            last_update: Instant::now(),
            bytes: 0,
        }
    }

    pub fn add(&mut self, size: u64) {
        self.bytes += size;

        if self.last_update.elapsed() >= Duration::from_millis(500) {
            self.pb.set_position(self.bytes);
            self.last_update = Instant::now();
        }
    }

    pub fn finish(self) {
        self.pb.finish_with_message("Done");
    }
}
