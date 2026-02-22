use std::ffi::OsStr;
use std::io::{BufRead, Write};

pub fn get_load_order(path: &std::path::Path) -> Vec<String> {
    let path = std::path::PathBuf::from(crate::MERGING_ROOT_PATH).join(path);

    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }

    if !path.exists() {
        japi::log_fatal!("Failed to create directory at:\n\"{}\"", path.display());
        return Vec::new();
    }

    let priority_path = std::path::PathBuf::from(&path).join("_load_order.cfg");

    let mut load_order: Vec<String> = Vec::new();

    if priority_path.exists() {
        let priority_file = std::fs::File::open(&priority_path).unwrap();
        for line in std::io::BufReader::new(priority_file).lines() {
            if let Ok(filename) = line {
                let item_path = std::path::PathBuf::from(&path).join(format!("{}.json", filename));
                if item_path.exists() {
                    load_order.push(filename);
                }
            }
        }
    }

    for entry in std::fs::read_dir(&path).unwrap() {
        if let Ok(entry) = entry {
            let entry_path = entry.path();
            if entry_path.extension() == Some(OsStr::new("json")) {
                let filename = entry_path.file_stem().unwrap().to_string_lossy().to_string();
                if !load_order.contains(&filename) {
                    load_order.insert(0, filename);
                }
            }
        }
    }

    let priority_output_file = std::fs::File::create(&priority_path).unwrap();
    let mut writer = std::io::BufWriter::new(priority_output_file);
    for filename in &load_order {
        writeln!(writer, "{}", filename).unwrap();
    }

    load_order
}

