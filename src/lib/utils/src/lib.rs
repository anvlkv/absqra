use std::path::{PathBuf};
use std::fs::{DirEntry, read_dir};
use std::ffi::OsStr;
use std::cmp::Ordering;

pub fn files_from_dir_recursively(path: PathBuf) -> Vec<DirEntry> {
    let mut entries = read_dir(path).unwrap().map(|f| f.unwrap()).collect::<Vec<DirEntry>>();

    entries.sort_by(|a_entry, b_entry| a_entry.path().partial_cmp(&b_entry.path()).unwrap()); // alphabetically

    entries.sort_by(|a_entry, b_entry| {                                                      // files first
        if a_entry.metadata().unwrap().is_dir() && b_entry.metadata().unwrap().is_file() {
            Ordering::Greater
        }
        else if a_entry.metadata().unwrap().is_file() && b_entry.metadata().unwrap().is_dir() {
            Ordering::Less
        }
        else {
            Ordering::Equal
        }
    });

    entries.into_iter()
        .flat_map(|entry| {
            match entry.metadata().unwrap() {
                meta if meta.is_dir() => files_from_dir_recursively(entry.path()),
                meta if meta.is_file() => vec![entry],
                _ => panic!("unsupported metadata")
            }
    }).collect()
}

pub fn for_each_ra_example_file<F>(mut func: F) 
where F: FnMut(DirEntry)  {
    let path = PathBuf::from("../../../examples");
    files_from_dir_recursively(path)
            .into_iter()
            .filter(|f| f.path().extension().and_then(OsStr::to_str).unwrap_or("") == "ra")
            .for_each(|entry| {
                println!("{:?}", entry.path());
                func(entry);
            });
}