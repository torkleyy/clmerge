use std::io::BufReader;
use std::fs::File;
use std::path::Path;

use ron::de::from_reader;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Config {
    /// The URL to insert before the PR number
    #[serde(default)]
    pub pull_request_prefix: String,
}

impl Config {
    pub fn from_path(path: &Path) -> ron::de::Result<Self> {
        from_reader(BufReader::new(File::open(path)?))
    }
}
