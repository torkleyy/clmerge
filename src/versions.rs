use std::collections::HashMap;

use semver::Version;

#[derive(Clone, Debug)]
pub struct Entries {
    pub map: HashMap<String, String>,
    pub version: Version,
}

pub struct Versions {
    pub map: HashMap<String, Entries>,
}

impl Versions {
    pub fn new() -> Self {
        Versions {
            map: Default::default(),
        }
    }

    pub fn add(&mut self, version: &Version) {
        self.map.insert(
            version.to_string(),
            Entries {
                map: Default::default(),
                version: version.clone(),
            },
        );
    }

    pub fn entry(&mut self, version: &Version, category: &str, line: &str) {
        let category = self
            .map
            .get_mut(&version.to_string())
            .unwrap()
            .map
            .entry(category.to_owned())
            .or_default();
        category.push_str(line.trim());
        category.push('\n');
    }
}
