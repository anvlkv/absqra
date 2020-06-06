use std::path::Path;
use std::fs::{DirEntry, read_dir};

pub fn files_from_dir_recursively(path: &str) -> Vec<DirEntry> {
    read_dir(Path::new(path)).unwrap().map(|f| f.unwrap()).flat_map(|entry| {
        match entry.metadata().unwrap() {
            meta if meta.is_dir() => files_from_dir_recursively(&entry.path().into_os_string().into_string().unwrap()),
            meta if meta.is_file() => vec![entry],
            _ => panic!("unsupported metadata")
        }
    }).collect()
}