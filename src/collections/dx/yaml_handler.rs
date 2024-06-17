use yaml_rust2::{Yaml, YamlLoader, YamlEmitter};
use std::error::Error;
use std::fmt;
use yaml_merge_keys::merge_keys;

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

fn get_error_snippet(yaml_content: &str, line: usize, col: usize) -> String {
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

pub fn load_yaml(yaml_str: &str) -> Result<Yaml, YamlMergeError> {
    let yamls = YamlLoader::load_from_str(yaml_str).map_err(|err| YamlMergeError::from((err, yaml_str.to_string())));
    join_yaml(yamls.unwrap())
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
            // Merge the keys.
            for doc in documents {
                let merged_yaml = merge_keys(doc.clone()).unwrap();
                let mut out_str = String::new();
                {
                    let mut emitter = YamlEmitter::new(&mut out_str);
                    emitter.dump(&merged_yaml).unwrap(); // dump the YAML object to a String
                    //emitter.dump(&doc).unwrap(); 
                }
                println!("{}", out_str);
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
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
    emitter.multiline_strings(true);
    emitter.dump(yaml)?;
    Ok(out_str)
}

// convert yaml_rust2::yaml to serde_json::Value
pub fn yaml_to_json(yaml: &Yaml) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let json = serde_json::to_value(&yaml.as_str())?;
    Ok(json)
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
        let yamls = YamlLoader::load_from_str(&file_data)?;
        let yaml = join_yaml(yamls)?;
        merged_yaml = merge_keys(yaml).unwrap();
    }
    let out_str = yaml_to_string(&merged_yaml).unwrap_or("".to_string());
    let _ = files_and_dirs::write_file(destination, &out_str)?;

    Ok(merged_yaml)
}