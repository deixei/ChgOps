use std::fs;
use std::error::Error;

pub fn list_all_files_and_dirs(path: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files = vec![];
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let path_str = path.to_str().unwrap().to_string();
        if path.is_file() {
            files.push(path_str);
        }
        else {
            let mut sub_files = list_all_files_and_dirs(path_str)?;
            files.append(&mut sub_files);
        } 
    }
    sort_files_by_path_length(files.as_mut());
    Ok(files)
}

pub fn find_files_by_regex(path: String, regex: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files = vec![];
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let path_str = path.to_str().unwrap().to_string();
        if path.is_file() {
            if path_str.contains(regex.as_str()) {
                files.push(path_str);
            }
        }
        else {
            let mut sub_files = find_files_by_regex(path_str, regex.clone())?;
            files.append(&mut sub_files);
        } 
    }
    sort_files_by_path_length(files.as_mut());
    Ok(files)
}

pub fn sort_files_by_path_length(files: &mut Vec<String>) {
    files.sort_by(|a, b| a.len().cmp(&b.len()));
}