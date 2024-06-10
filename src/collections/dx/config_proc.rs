use serde_yaml;
use serde_yaml::Mapping;
use serde_json;
use serde_yaml::Value as YamlValue;
use serde_json::Value as JsonValue;
use std::error::Error;

use std::fs;
use tera::{Tera, Context};
use std::collections::BTreeMap;
use regex::Regex;

use super::core::filters;

/// Reads a YAML file from the specified file path and returns the parsed YAML value.
pub fn read_yaml(file_path: &str) -> Result<serde_yaml::Value, Box<dyn std::error::Error>> {
    // check if file exists before reading, else return error
    if !std::path::Path::new(file_path).exists() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, format!("File not found: {}", file_path))));
    }
    let content = fs::read_to_string(file_path)?;
    let yaml: serde_yaml::Value = serde_yaml::from_str(&content)?;
    Ok(yaml)
}

/// Converts a YAML value to a string representation.
pub fn yaml_to_string(yaml: &serde_yaml::Value) -> String {
    serde_yaml::to_string(yaml).unwrap()
}

/// Converts a YAML value to a JSON value.
pub fn yaml_to_json(yaml_value: &YamlValue) -> Result<JsonValue, Box<dyn Error>> {
    let json_value: JsonValue = serde_json::to_value(yaml_value)?;
    Ok(json_value)
}

/// Processes a template string using the provided context and returns the rendered result.
pub fn process_template(template_str: &str, context: &Context) -> Result<String, tera::Error> {
    let mut tera = Tera::default();
    let _ = tera.add_raw_template("process_template", &template_str);
    tera.register_function("current_time", filters::current_time());
    tera.register_function("env_var", filters::env_var());
    tera.register_filter("filter1", filters::filter1);
    tera.register_filter("filter2", filters::filter2);

    tera.render("process_template", &context)
}

/// Merges two YAML values recursively.
pub fn merge_yaml(a: &mut YamlValue, b: YamlValue) {
    match (a, b) {
        (YamlValue::Mapping(a_map), YamlValue::Mapping(b_map)) => {
            for (k, v) in b_map {
                merge_yaml(a_map.entry(k).or_insert(YamlValue::Null), v);
            }
        }
        (a_val, b_val) => {
            *a_val = b_val;
        }
    }
}

/// Resolves references in a YAML value using a map of reference values.
fn resolve_references(value: &mut YamlValue, references: &BTreeMap<String, YamlValue>) {
    let re = Regex::new(r"\{\{ ref:([a-zA-Z0-9_]+) \}\}").unwrap();
    match value {
        YamlValue::String(s) => {
            *s = re.replace_all(s, |caps: &regex::Captures| {
                references.get(&caps[1]).unwrap_or(&YamlValue::Null).as_str().unwrap_or("").to_string()
            }).to_string();
        }
        YamlValue::Mapping(map) => {
            for val1 in map.iter_mut() {
                resolve_references(val1.1, references);
            }
        }
        YamlValue::Sequence(seq) => {
            for val in seq.iter_mut() {
                resolve_references(val, references);
            }
        }
        _ => {}
    }
}

/// Processes configuration files by merging them, resolving references, and rendering templates.
pub fn process_configuration_files(collections_files: Vec<String>, workplace_files: Vec<String>) -> Result<YamlValue, Box<dyn std::error::Error>> {
    //println!("Processing configuration files...");
    let file_paths: Vec<String> = collections_files.into_iter().chain(workplace_files).collect();
    let mut merged_yaml = YamlValue::Mapping(Mapping::new());
    let mut references = BTreeMap::new();

    for file_path in &file_paths {
        //println!("Processing file: {}", file_path);
        let yaml: YamlValue = read_yaml(file_path)?;
        merge_yaml(&mut merged_yaml, yaml.clone());

        // Collect references if any (this part depends on your reference structure)
        if let YamlValue::Mapping(map) = &yaml {
            for (k, v) in map {
                if let YamlValue::String(s) = k {
                    references.insert(s.clone(), v.clone());
                }
            }
        }
    }

    resolve_references(&mut merged_yaml, &references);

    let template_str = yaml_to_string(&merged_yaml);
    let json_value: JsonValue = yaml_to_json(&merged_yaml).unwrap();
    let context = Context::from_value(json_value).unwrap();

    let computed_str = process_template(&template_str, &context);

    let final_yaml: YamlValue = serde_yaml::from_str(&computed_str.unwrap())?;

    Ok(final_yaml)
}
