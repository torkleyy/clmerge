use std::{collections::HashMap, fs::write, path::Path};

use crate::versions::Entries;
use crate::config::Config;

pub struct ClWriter {
    buffer: String,
}

impl ClWriter {
    pub fn new(header: &str) -> Self {
        let mut this = ClWriter {
            buffer: String::new(),
        };

        this.write_str("<!-- This file is auto-generated. Do not edit. -->\n");
        this.write_str(header);
        this.write_str("\n");

        this
    }

    pub fn write_versions(&mut self, versions: &[Entries]) {
        self.write_str("\n");
        for version in versions {
            self.write_version_header(&version.version.to_string());
            self.write_version(&version.map);
        }
    }

    fn write_version_header(&mut self, version: &str) {
        self.write_str("## ");
        self.write_str(version);
        self.write_str("\n\n");
    }

    fn write_version(&mut self, categories: &HashMap<String, String>) {
        for (header, lines) in categories {
            let header = crate::util::capitalize(header);
            self.write_category(&header, lines);
        }
    }

    fn write_category(&mut self, category: &str, content: &str) {
        self.write_str("### ");
        self.write_str(category);
        self.write_str("\n\n");
        self.write_str(content);
        self.write_str("\n");
    }

    pub fn write_str(&mut self, value: &str) {
        self.buffer.push_str(value);
    }

    pub fn trim(&mut self) {
        while let Some(index) = self.buffer.find("\n\n\n") {
            let mut exclusive_end = index + 3;
            while let Some(b'\n') = self.buffer.as_bytes().get(exclusive_end) {
                exclusive_end += 1;
            }

            self.buffer.replace_range(index..exclusive_end, "\n\n");
        }
    }

    pub fn linkify_prs(&mut self, config: &Config) {
        let regex = regex::Regex::new(r"\[#(\d*)\]").unwrap();

        let mut start = 0;
        while let Some(m) = regex.find_at(&self.buffer, start) {
            if config.pull_request_prefix.is_empty() {
                eprintln!("Error: cannot replace PR link because there is no PR link prefix in `config.ron`");
                return;
            }

            start = m.end();

            let start = m.start();
            let end = m.end();

            let orig = &self.buffer[start..end];
            let new = format!("{}({}{})", orig, &config.pull_request_prefix, &orig[2..orig.len() - 1]);

            self.buffer.replace_range(start..end, &new);
        }
    }

    pub fn write_to_file(&self, path: &Path) -> std::io::Result<()> {
        write(path, &self.buffer)?;

        Ok(())
    }
}
