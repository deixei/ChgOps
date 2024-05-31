use serde::{Deserialize, Serialize};
use crate::collections::dx::azure::cli::AzCli;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Tasks {
    AzureLoginTask {
        name: String,
        #[serde(rename = "dx.azure.login")]
        login: serde_yaml::Value,
        register: Option<String>,
    },
    AzureCliTask {
        name: String,
        #[serde(rename = "dx.azure.cli")]
        az_cli: AzCli,
        register: Option<String>,
    },
}



pub fn execute(task) {

    match task {
        Tasks::AzureCliTask { name, az_cli, .. } => {
            println!("Running task: {}", name);
            let bash = AzCli::new(&az_cli.command);
            let output = bash.execute().expect("Failed to execute command");
            bash.display(output);
        }
        Tasks::AzureLoginTask { name, .. } => {
            println!("Running task: {}", name);
            // Handle the LoginTask variant here
        }
        
        
    }
}