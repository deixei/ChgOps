
use std::fs;
use serde_yaml;
use std::env;
use crate::collections::dx::{Playbook, EngineParameters};


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
    let mut playbook: Playbook = serde_yaml::from_str(&playbook_yaml)
        .expect("Failed to parse playbook");


    playbook.set_engine_parameters(params);

    playbook.display();

    playbook.run_tasks();

    playbook.display_summary();

}