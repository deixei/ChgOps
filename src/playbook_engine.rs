use serde::{Deserialize, Serialize};
use std::fs;
use serde_yaml;
use std::env;


#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum PlaybookTasks {
    CoreTasks(crate::collections::dx::core::tasks::Tasks),
    AzureTasks(crate::collections::dx::azure::tasks::Tasks),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Task {
    BashCommandTask {
        name: String,
        #[serde(rename = "dx.core.bash")]
        command: String,
        register: Option<String>,
    },
    WinCmdCommandTask {
        name: String,
        #[serde(rename = "dx.core.wincmd")]
        command: String,
        register: Option<String>,
    },
    AzureLoginTask {
        name: String,
        #[serde(rename = "dx.azure.login")]
        login: serde_yaml::Value,
        register: Option<String>,
    },
    AzureCliTask {
        name: String,
        #[serde(rename = "dx.azure.cli")]
        task: AzCliParameters,
        register: Option<String>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
struct AzCliParameters {
    cmd: String,
}

#[derive(Debug, Deserialize)]
struct Settings {
    name: String
}

#[derive(Debug, Deserialize)]
struct Playbook {
    name: String,
    settings: Settings,
    tasks: Vec<PlaybookTasks>,
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
        crate::collections::dx::core::tasks::execute(task);
        crate::collections::dx::azure::tasks::execute(task);
    }
}