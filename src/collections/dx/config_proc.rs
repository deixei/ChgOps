use serde_yaml;
use serde_yaml::{Value, Mapping};

use std::fs;
use tera::{Tera, Context};
use std::collections::BTreeMap;
use regex::Regex;


fn read_yaml(file_path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let yaml: Value = serde_yaml::from_str(&content)?;
    Ok(yaml)
}

fn process_template(template_str: &str, context: &Context) -> Result<String, tera::Error> {
    let mut tera = Tera::default(); // Or load templates from a directory
    tera.render_str(template_str, context)
}

fn merge_yaml(a: &mut Value, b: Value) {
    match (a, b) {
        (Value::Mapping(a_map), Value::Mapping(b_map)) => {
            for (k, v) in b_map {
                merge_yaml(a_map.entry(k).or_insert(Value::Null), v);
            }
        }
        (a_val, b_val) => {
            *a_val = b_val;
        }
    }
}

fn resolve_references(value: &mut Value, references: &BTreeMap<String, Value>) {
    let re = Regex::new(r"\{\{ ref:([a-zA-Z0-9_]+) \}\}").unwrap();
    match value {
        Value::String(s) => {
            *s = re.replace_all(s, |caps: &regex::Captures| {
                references.get(&caps[1]).unwrap_or(&Value::Null).as_str().unwrap_or("").to_string()
            }).to_string();
        }
        Value::Mapping(map) => {
            for val1 in map.iter_mut() {
                let val = val1.1;
                resolve_references(val, references);
            }
        }
        Value::Sequence(seq) => {
            for val in seq.iter_mut() {
                resolve_references(val, references);
            }
        }
        _ => {}
    }
}

pub fn process_configuration_files() -> Result<(), Box<dyn std::error::Error>> {
    println!("Processing configuration files...");
    let file_paths = vec![
            "/home/marcio/repos/deixei/ChgOps/collections/dx/azure/vars.yaml", 
            "/home/marcio/repos/deixei/ChgOps/collections/dx/azure/blueprint1/vars.yaml", 
            "/home/marcio/repos/deixei/ChgOps/collections/dx/azure/blueprint2/vars.yaml",
            "./playbooks/workspace2/vars.yaml"];
            //"./playbooks/workspace2/plyreg1.yaml"];
    let mut merged_yaml = serde_yaml::Value::Mapping(Mapping::new());
    let mut references = BTreeMap::new();

    for file_path in &file_paths {
        let yaml = read_yaml(file_path)?;
        // Assume each file can contain templates that need to be processed
        let context = Context::new();
        let yaml_str = serde_yaml::to_string(&yaml)?;
        let processed_str = process_template(&yaml_str, &context)?;
        let processed_yaml: Value = serde_yaml::from_str(&processed_str)?;
        
        merge_yaml(&mut merged_yaml, processed_yaml.clone());

        // Collect references if any (this part depends on your reference structure)
        if let Value::Mapping(map) = &processed_yaml {
            for (k, v) in map {
                if let Value::String(s) = k {
                    references.insert(s.clone(), v.clone());
                }
            }
        }
    }

    // Resolve references in the merged YAML
    resolve_references(&mut merged_yaml, &references);

    println!("COMPUTED: {}", serde_yaml::to_string(&merged_yaml)?);
    Ok(())
}
