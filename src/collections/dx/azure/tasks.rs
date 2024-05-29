use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Tasks {
    AzureLoginTask {
        name: String,
        #[serde(rename = "dx.azure.login")]
        login: serde_yaml::Value,
        register: Option<String>,
    },
    CliTask {
        name: String,
        #[serde(rename = "dx.azure.cli")]
        az_cli: AzCli,
        register: Option<String>,
    },
}



pub fun execute(task){

    match task {

        
    }
}