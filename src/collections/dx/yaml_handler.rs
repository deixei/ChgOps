use yaml_rust2::{Yaml, YamlLoader, YamlEmitter};
use std::error::Error;
use std::fmt;
use yaml_merge_keys::merge_keys;
use serde_yaml;
use serde_json;

use crate::print_error;
//use crate::{print_error, print_info, print_success, print_warning};

// Define a custom error type for better error handling
#[derive(Debug)]
pub enum YamlMergeError {
    ParseError(yaml_rust2::ScanError, String),
    // Add other variants as needed
}

impl fmt::Display for YamlMergeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            YamlMergeError::ParseError(err, yaml_content) => {
                let error_location = format!("line {}, column {}", err.marker().line().to_string(), err.marker().col().to_string());
                let snippet = get_error_snippet(yaml_content, err.marker().line(), err.marker().col());
                write!(f, "YAML Parse Error at {}: {}\nSnippet:\n{}", error_location, err, snippet)
            },
            // Handle other variants
        }
    }
}

impl Error for YamlMergeError {}

impl From<(yaml_rust2::ScanError, String)> for YamlMergeError {
    fn from(err: (yaml_rust2::ScanError, String)) -> YamlMergeError {
        YamlMergeError::ParseError(err.0, err.1)
    }
}

pub fn get_error_snippet(yaml_content: &str, line: usize, _col: usize) -> String {
    let lines: Vec<&str> = yaml_content.lines().collect();
    let start = if line > 2 { line - 2 } else { 0 };
    let end = if line + 3 < lines.len() { line + 3 } else { lines.len() };
    
    let snippet: Vec<String> = lines[start..end]
        .iter()
        .enumerate()
        .map(|(i, l)| {
            if i + start == line {
                format!("{} --> {}", i + start + 1, l)
            } else {
                format!("{}     {}", i + start + 1, l)
            }
        })
        .collect();
    snippet.join("\n")
}

pub fn combine_yaml(a: &mut Yaml, b: Yaml) {
    match (a, b) {
        (Yaml::Hash(a), Yaml::Hash(b)) => {
            for (key, value) in b.iter() {
                if a.contains_key(key) {
                    let a_val = a.get_mut(key).unwrap();
                    combine_yaml(a_val, value.clone());
                } else {
                    a.insert(key.clone(), value.clone());
                }
            }
        },
        (a_val, b_val) => {
            *a_val = b_val.clone();
        }
    }
}

pub fn load_yaml(yaml_str: &str) -> Result<Yaml, YamlMergeError> {
    let yaml = YamlLoader::load_from_str(&yaml_str).map_err(|err| YamlMergeError::from((err, yaml_str.to_string())));
    match yaml {
        Ok(yaml) => {
            Ok(join_yaml(yaml)?)
        },
        Err(e) => Err(e),
    }
}


pub fn example() -> Result<(), Box<dyn std::error::Error>> {
    println!("Merged YAML:");
    let yaml1 = r#"
---    
ref: &ref
    merged_key: merged
    added_key: merged
l: &l
    - name: a
      value: 2
    - name: b
      value: 3
    - name: c
      value: 4
name: &name 
    description: marcio
"#;

    let yaml2 = r#"

dict:
    <<: *ref
    top_key: given
    merged_key: given
    l: *l
--- 
x:
    description: "{{ name.description }}"
"#;

    // Combine the two YAML strings
    let mut raw_yaml = yaml1.to_string();
    raw_yaml += "\n";
    raw_yaml += yaml2;

    match load_yaml(&raw_yaml) {
        Ok(documents) => {
            let out_str = yaml_to_string_pretty(&documents)?;
            println!("{}", out_str);
        },
        Err(e) => {
            print_error!("{}", e);
        }
        
    }

    Ok(())
}



pub fn yaml_to_string(yaml: &Yaml) -> Result<String, Box<dyn std::error::Error>> {
    let mut out_str = String::new();
    let mut emitter = YamlEmitter::new(&mut out_str);
    emitter.dump(yaml)?;
    Ok(out_str)
}

pub fn yaml_to_string_pretty(yaml: &Yaml) -> Result<String, Box<dyn std::error::Error>> {
    let mut out_str = String::new();
    let mut emitter = YamlEmitter::new(&mut out_str) ;
    emitter.compact(false);
    emitter.multiline_strings(true);
    emitter.dump(yaml)?;
    Ok(out_str)
}

// convert yaml string to serde_json::Value
pub fn yaml_to_json(yaml_str: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(yaml_str).unwrap();

    match serde_json::to_value(yaml_value){
        Ok(json) => {
            //println!("{:#?}", json);
            Ok(json)},
        Err(e) => {
            let message = format!("Converting YAML to JSON: {:#?}", e);
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, message)))
        }        
    }
}

// join Vec<Yaml> into a single Yaml object
pub fn join_yaml(yamls: Vec<Yaml>) -> Result<Yaml, YamlMergeError> {
    let mut merged_yaml = Yaml::Null;
    for yaml in yamls {
        merged_yaml = merge_keys(yaml).unwrap();
    }
    Ok(merged_yaml)
}

use crate::collections::dx::files_and_dirs;

pub fn load(files: Vec<String>, destination: &str) -> Result<Yaml, Box<dyn std::error::Error> > {
    let mut merged_yaml = Yaml::Null;
    for file in files {
        let file_data = files_and_dirs::read_file(&file)?;
        match load_yaml(&file_data) {
            Ok(documents) => {
                combine_yaml(&mut merged_yaml, merge_keys(documents).unwrap());
            },
            Err(e) => {
                print_error!("Error: {} \n File: {}", e, file);
            }
        }
    }
    let out_str = yaml_to_string_pretty(&merged_yaml).unwrap_or("".to_string());
    let _ = files_and_dirs::write_file(destination, &out_str)?;

    Ok(merged_yaml)
}