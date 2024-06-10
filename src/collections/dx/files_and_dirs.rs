use std::fs;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

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

pub fn find_files_by_regex(path: String, re: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let regex = Regex::new(re).unwrap();
    let mut files = vec![];
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let path_str = path.to_str().unwrap().to_string();
        if path.is_file() {
            //println!("Checking file: {}", path_str);
            if regex.is_match(path_str.as_str()) {
                //println!("Matched file: {}", path_str);
                files.push(path_str);
            }
        }
        else {
            let mut sub_files = find_files_by_regex(path_str, re)?;
            files.append(&mut sub_files);
        } 
    }
    sort_files_by_path_length(files.as_mut());
    Ok(files)
}

pub fn sort_files_by_path_length(files: &mut Vec<String>) {
    files.sort_by(|a, b| a.len().cmp(&b.len()));
}


pub fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    s
}