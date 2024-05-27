use serde::Deserialize;
use std::fs;
use serde_yaml;
use std::env;
use crate::collections::dx::core::shell::Bash;


#[derive(Debug, Deserialize)]
struct Task {
    name: String,
    command: String,
}

#[derive(Debug, Deserialize)]
struct Settings {
    name: String
}

#[derive(Debug, Deserialize)]
struct Playbook {
    name: String,
    settings: Settings,
    tasks: Vec<Task>,
}


#[derive(Debug, Deserialize)]
pub struct EngineParameters {
    pub playbook_name: String,
    pub workspace_path: String,
    pub verbose: String,
    pub arguments: String,
    
}

pub fn engine_run(params: EngineParameters) {
    
    let current_dir = env::current_dir().unwrap().to_str().unwrap().to_owned();
    println!("The current directory is {}", current_dir.to_string());
    
    
    let workspace_path = if params.workspace_path.is_empty() {
        current_dir
    } else {
        params.workspace_path.clone()
    };

    let playbook_full_path = format!("{}/{}.yaml", workspace_path, params.playbook_name);

    let playbook_yaml = fs::read_to_string(playbook_full_path)
        .expect("Failed to read playbook");

    let playbook: Playbook = serde_yaml::from_str(&playbook_yaml)
        .expect("Failed to parse playbook");

    println!("Playbook name: {}", playbook.name);

    println!("Settings name: {}", playbook.settings.name);

    for task in playbook.tasks {
        println!("Running task: {}", task.name);
        let bash = Bash::new(&task.command);
        let output = bash.execute().expect("Failed to execute command");
        bash.display(output);

    }
}