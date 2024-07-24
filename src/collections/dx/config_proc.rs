use serde_yaml;
use serde_json;
use serde_yaml::Value as YamlValue;
use serde_json::Value as JsonValue;
use std::error::Error;
use std::fs;
use tera::{Tera, Context};
use crate::collections::{dx::yaml_handler};
use super::{core::filters, files_and_dirs};
use std::path::Path;
use crate::{print_error, print_info, print_success, print_warning};
/// Reads a YAML file from the specified file path and returns the parsed YAML value.
pub fn read_yaml(file_path: &str) -> Result<serde_yaml::Value, Box<dyn Error>> {
    // Check if the file exists before attempting to read it
    if !Path::new(file_path).exists() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", file_path),
        )));
    }

    // Read the file content to a string
    let content = fs::read_to_string(file_path)?;

    // Try to parse the string content as YAML
    match serde_yaml::from_str(&content) {
        Ok(yaml) => Ok(yaml),
        Err(e) => {
            // If there's an error, gather details about where the error occurred
            if let Some(location) = e.location() {
                let line = location.line() + 1;
                let column = location.column() + 1;
                let snippet = content.lines().nth(location.line()).unwrap_or("");
                let message = format!(
                    "Reading and deserializing YAML file: {} at line {} column {}: {:#?} - [{:#?}]",
                    file_path, line, column, snippet, e
                );
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, message)))
            } else {
                // If there's no specific location, return a generic error message
                let message = format!("Reading and deserializing YAML file: {} - [{}]", file_path, e);
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, message)))
            }
        }
    }
}



/// Converts a YAML value to a string representation.
pub fn yaml_to_string(yaml: &serde_yaml::Value) -> Result<String, Box<dyn Error>> {
    // handle errors
    match serde_yaml::to_string(yaml) {
        Ok(s) => Ok(s),
        Err(e) => {
            let message = format!("Converting YAML to string: {}", e);
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, message)))
        }
    }

}



/// Processes a template string using the provided context and returns the rendered result.
pub fn process_template(template_str: &str, context: &Context) -> Result<String, tera::Error> {
    let mut tera = Tera::default();
    let _ = tera.add_raw_template("process_template", &template_str);

    tera.register_function("current_time", filters::current_time());
    tera.register_function("env_var", filters::env_var());
    tera.register_filter("filter1", filters::filter1);
    tera.register_filter("filter2", filters::filter2);
    tera.register_filter("as_yaml", filters::as_yaml);
    tera.register_filter("as_json", filters::as_json);
    tera.register_filter("as_base64", filters::as_base64);

    match tera.render("process_template", &context) {
        Ok(rendered) => {
            // Use the rendered template
            Ok(rendered)
        },
        Err(e) => {
            let error : tera::Error = e;
            print_error!("Rendering template: {:#?}", error);
            // Handle the error
            Err(error)
        }
    }
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


/// Processes configuration files by merging them, resolving references, and rendering templates.
pub fn process_configuration_files(collections_files: Vec<String>, workplace_files: Vec<String>) -> Result<yaml_rust2::Yaml, Box<dyn std::error::Error>> {
    //println!("Processing configuration files...");
    let file_paths: Vec<String> = collections_files.into_iter().chain(workplace_files).collect();
    let _yaml: yaml_rust2::Yaml = yaml_handler::load(file_paths, "./temp/merged.yaml")?;

    let file_data = files_and_dirs::read_file("./temp/merged.yaml")?;
    let json: JsonValue = yaml_handler::yaml_to_json(&file_data).unwrap();
    //println!("JSON: {:#?}", json);
    //let tera_context = Context::from_value(json)?;
    let tera_context = Context::from_serialize(json)?;
    //println!("tera_context: {:#?}", tera_context);
    //println!("file_data: {:#?}", file_data);
    let _r = process_template(&file_data, &tera_context)?;
    let _o = files_and_dirs::write_file("./temp/final.yaml", &_r)?;

    let merged_yaml = yaml_handler::load_yaml(&_r)?;

    Ok(merged_yaml)
}


/// Loads a YAML file from the specified file path and returns the parsed YAML value.
pub fn process_playbook(file_path: &str, current_config_yaml: yaml_rust2::Yaml) -> Result<String, Box<dyn Error>> {
    let file_data = files_and_dirs::read_file(file_path)?;
    let mut merged_yaml = current_config_yaml.clone();

    match yaml_handler::load_yaml(&file_data) {
        Ok(yaml) => {
            yaml_handler::combine_yaml(&mut merged_yaml, yaml);
        },
        Err(e) => {
            print_error!("{}", e);
        }
    }
    let _playbook_yaml_file_out = files_and_dirs::write_file("./temp/playbook_s1.yaml", &yaml_handler::yaml_to_string(&merged_yaml)?)?;
    
    let template = files_and_dirs::read_file("./temp/playbook_s1.yaml")?;

    Ok(template)
}

// based on an input of "{{ object.path }}", returns "object.path" trimmed
pub fn extract_object_path_from_handlebars(handlebars: &str) -> String {
    // if contains pipes,  return the original string
    let mut path = handlebars.to_string();
    if path.contains("|") {
        return path;
    }
    // extract the path from inside the handlebars, with a regex
    let re = regex::Regex::new(r"\{\{(.*)\}\}").unwrap();
    let caps = re.captures(&handlebars).unwrap();
    path = caps.get(1).unwrap().as_str().to_string();
    path = path.trim().to_string();
    path
}