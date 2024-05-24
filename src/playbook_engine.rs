use serde::Deserialize;
use std::process::Command;
use std::fs;
use serde_yaml;

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

pub fn engine_run(name: String, template: String) {
    
    let playbook_yaml = fs::read_to_string(name)
        .expect("Failed to read playbook");

    let playbook: Playbook = serde_yaml::from_str(&playbook_yaml)
        .expect("Failed to parse playbook");

    println!("Playbook name: {}, using template: {}", playbook.name, template);

    println!("Settings name: {}", playbook.settings.name);

    for task in playbook.tasks {
        println!("Running task: {}", task.name);
        let output = Command::new("sh")
            .arg("-c")
            .arg(&task.command)
            .output()
            .expect("Failed to execute command");

        if !output.status.success() {
            eprintln!("Task '{}' failed with exit code: {}", task.name, output.status);
            break;
        }
        else {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let output_str = output_str.replace("\\n", "\n");
            println!("Output: {}", output_str);
        }
    }
}