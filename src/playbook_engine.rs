
use std::fs;
use serde_yaml;
use std::env;
use crate::collections::dx::{Playbook, PlaybookCommandTrait, PlaybookTasks, EngineParameters};


pub fn engine_run(params: EngineParameters) {
    println!("#############################################");
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
    println!("#############################################");

    let mut task_counter = 0;
    let mut success_counter = 0;
    let mut failed_counter = 0;
    let mut skipped_counter = 0;
    let mut changed_counter = 0;

    for item in playbook.tasks {

        task_counter += 1;

        match item {

            PlaybookTasks::CoreTasks(task) => {
                println!("************************************");
                println!("Core Task: {:?}", task);
                let output = task.execute().expect("Failed to execute task");
                task.display(output);
                println!("************************************");
            },

            PlaybookTasks::AzureTasks(task) => {
                println!("************************************");
                println!("Azure Task: {:?}", task);
                let output = task.execute().expect("Failed to execute task");
                task.display(output);
                println!("************************************");
            },

            
        }
    }
    println!("#############################################");
    println!("Playbook execution completed");
    println!("Total tasks executed: {}", task_counter);
    println!("Total tasks success: {}", success_counter);
    println!("Total tasks failed: {}", failed_counter);
    println!("Total tasks skipped: {}", skipped_counter);
    println!("Total tasks changed: {}", changed_counter);
    println!("#############################################");
}