use std::path::{PathBuf};
use std::fs::{DirEntry, read_dir};
use std::ffi::OsStr;

pub fn files_from_dir_recursively(path: PathBuf) -> Vec<DirEntry> {
    read_dir(path).unwrap().map(|f| f.unwrap()).flat_map(|entry| {
        match entry.metadata().unwrap() {
            meta if meta.is_dir() => files_from_dir_recursively(entry.path()),
            meta if meta.is_file() => vec![entry],
            _ => panic!("unsupported metadata")
        }
    }).collect()
}

pub fn for_each_ra_example_file<F>(func: F) 
where F: FnMut(DirEntry)  {
    let path = PathBuf::from("../../../examples");
    files_from_dir_recursively(path)
            .into_iter()
            .filter(|f| f.path().extension().and_then(OsStr::to_str).unwrap_or("") == "ra")
            .for_each(func);
}