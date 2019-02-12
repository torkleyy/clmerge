use std::{fs::read_to_string, io::ErrorKind, path::Path};

pub fn file_to_string(path: &Path) -> String {
    match read_to_string(path) {
        Ok(file) => file,
        Err(e) => error_exit!("Failed to read file `{}`: {}", path.display(), e),
    }
}

pub fn file_to_string_else_empty(path: &Path) -> String {
    match read_to_string(path) {
        Ok(file) => file,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                return String::new();
            }

            error_exit!("Failed to read file `{}`", path.display())
        }
    }
}

pub fn iter_dir(path: &Path) -> Vec<std::fs::DirEntry> {
    let entries = match path.read_dir() {
        Ok(x) => x,
        Err(e) => error_exit!("Failed to read `{}` directory: {}", path.display(), e),
    };

    match entries.collect::<Result<Vec<_>, _>>() {
        Ok(x) => x,
        Err(e) => error_exit!(
            "Failed to read one entry of dir `{}`: {}",
            path.display(),
            e
        ),
    }
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}
