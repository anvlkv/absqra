use std::path::{PathBuf};
use std::fs::{DirEntry, read_dir, File};
use std::ffi::OsStr;
use std::cmp::Ordering;
use std::io::Read;
use std::env::{current_dir};

pub fn files_from_dir_recursively(path: PathBuf) -> Vec<DirEntry> {
    let mut entries: Vec<DirEntry> = match read_dir(path.clone()) {
        Ok(r) => r.map(|f| f.unwrap()).collect(),
        Err(_e) => {
            panic!(format!("failed to get example: {:?}, from: {:?}", path.as_os_str(), current_dir()));
        }
    };

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
where F: FnMut(String, String)  {
    let mut path = current_dir().unwrap().to_path_buf();
    path.push(PathBuf::from("examples/"));

    let dir_path_length = path.to_str().unwrap().len();
    
    files_from_dir_recursively(path)
        .into_iter()
        .filter(|f| f.path().extension().and_then(OsStr::to_str).unwrap_or("") == "ra")
        .for_each(|entry| {
            let mut file = {
                match File::open(entry.path()) {
                    Err(e) => {
                        panic!(format!("{:?}", e));
                    }
                    Ok(f) => f
                }
            };
            
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            let file_path = String::from(entry.path().to_str().unwrap());

            let file_name = unsafe {
                file_path
                    .get_unchecked(dir_path_length .. file_path.len())
                    .to_string()
                    .replace("/", "_")
                    .replace(".", "_")
                    .replace("-", "_")

            };

            func(contents, file_name);
        });
}