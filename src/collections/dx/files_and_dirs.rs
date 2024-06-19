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


pub fn read_file(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    if !std::path::Path::new(file_path).exists() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, format!("ERROR: File not found: {}", file_path))));
    }
    match std::fs::read_to_string(file_path) {
        Ok(s) => {
            println!("Reading file: {}", file_path);
            Ok(s)
        },
        Err(e) => {
            let message = format!("ERROR: Reading file: {}, {}", e, file_path);
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, message)))
        }
    }
}

pub fn write_file(file_path: &str, content: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn append_file(file_path: &str, content: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::OpenOptions::new().append(true).open(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}


pub fn join_files(files: Vec<String>) -> Result<String, Box<dyn Error>> {
    let mut content = String::new();
    for file in files {
        content.push_str(&read_file(&file)?);
    }
    Ok(content)
}

pub fn copy_file(src: &str, dest: &str) -> Result<(), Box<dyn Error>> {
    fs::copy(src, dest)?;
    Ok(())
}

pub fn join_files_into(files: Vec<String>, separator: &str, dest: &str) -> Result<(), Box<dyn Error>> {
    let mut content = String::new();
    for file in files {
        content.push_str(&read_file(&file)?);
        content.push_str(separator);
    }
    write_file(dest, &content)?;
    Ok(())
}