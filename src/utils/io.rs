use std::{fs, path::PathBuf};

pub fn get_files(path: &PathBuf) -> Vec<(String, bool)> {
    let mut dirs = Vec::default();
    let mut files = Vec::default();

    if let Ok(read_dir) = fs::read_dir(path) {
        for dir_entry in read_dir.flatten() {
            if let Some(name) = dir_entry.file_name().to_str() {
                if dir_entry.path().is_dir() && !name.starts_with(".") {
                    dirs.push((name.to_string(), true));
                } else if name.ends_with(".mkv") || name.ends_with(".MKV") {
                    files.push((name.to_string(), false));
                }
            }
        }
    }

    dirs.append(&mut files);
    dirs
}

pub fn get_side_bar_dirs_and_files(path: &PathBuf) -> Vec<(String, bool)> {
    let mut dirs = Vec::default();

    if let Ok(read_dir) = fs::read_dir(path) {
        for dir_entry in read_dir.flatten() {
            if let Some(name) = dir_entry.file_name().to_str()
                && dir_entry.path().is_dir()
                && !name.starts_with(".")
            {
                dirs.push((name.to_string(), true));
            }
        }
    }

    dirs
}
