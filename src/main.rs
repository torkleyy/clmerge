use std::{ffi::OsString, path::PathBuf};

use semver::Version;

use crate::{
    config::Config,
    cl_writer::ClWriter,
    util::{file_to_string, file_to_string_else_empty, iter_dir},
    versions::Versions,
};

mod config;
mod cl_writer;
#[macro_use]
mod macros;
mod util;
mod versions;

fn main() {
    let changelog = PathBuf::from("changelog");

    if !changelog.is_dir() {
        error_exit!("No `changelog` directory");
    }

    let config = Config::from_path(&changelog.join("config.ron")).unwrap_or_else(|e| {
        println!("Could not parse `changelog/config.ron`: {}", e);

        Default::default()
    });

    let mut versions = Versions::new();

    let old = file_to_string_else_empty(&changelog.join("old.md"));
    let header = file_to_string_else_empty(&changelog.join("header.md"));

    for version_dir in iter_dir(&changelog) {
        if !version_dir.path().is_dir() {
            continue;
        }

        let file_name: OsString = version_dir.file_name();
        let name = match file_name.to_str() {
            Some(x) => x,
            None => {
                error_exit!(
                    "Found directory which is neither semver nor UTF-8: {}",
                    version_dir.path().display()
                );
            }
        };

        let version = match Version::parse(name) {
            Ok(x) => x,
            Err(e) => {
                error_exit!(
                    "Directory is not semver-conformant: {}: {}",
                    version_dir.path().display(),
                    e
                );
            }
        };

        versions.add(&version);

        for category_dir in iter_dir(&version_dir.path()) {
            let file_name: OsString = category_dir.file_name();
            let category = match file_name.to_str() {
                Some(x) => x,
                None => {
                    error_exit!(
                        "Found category which is no valid UTF-8: {}",
                        category_dir.path().display()
                    );
                }
            };

            for entry_file in iter_dir(&category_dir.path()) {
                versions.entry(&version, category, &file_to_string(&entry_file.path()));
            }
        }
    }

    let mut entries = versions.map.into_iter().map(|(_, v)| v).collect::<Vec<_>>();
    entries.sort_by(|a, b| b.version.cmp(&a.version));

    //dbg!(&entries);

    let mut writer = ClWriter::new(&header);
    writer.write_versions(&entries);
    writer.write_str(&old);
    writer.trim();
    writer.linkify_prs(&config);

    if let Err(e) = writer.write_to_file("CHANGELOG.md".as_ref()) {
        error_exit!("Failed to write `CHANGELOG.md`: {}", e);
    }
}
