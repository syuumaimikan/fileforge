use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use fileforge_core::Result;

#[derive(Debug, Clone)]
pub struct FileProbe {
    pub path: PathBuf,
    pub extension: Option<String>,
    pub size: u64,
    pub header: Vec<u8>,
}

impl FileProbe {
    pub fn from_path(path: &Path) -> Result<Self> {
        let mut file = File::open(path)?;
        let size = file.metadata()?.len();

        let mut header = vec![0u8; 4096];
        let read_size = file.read(&mut header)?;
        header.truncate(read_size);

        let extension = path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_ascii_lowercase());

        Ok(Self {
            path: path.to_path_buf(),
            extension,
            size,
            header,
        })
    }
}
