use serde::{Deserialize, Serialize};
use crate::collections::dx::{azure::cli::AzCli, PlaybookCommand, PlaybookCommandTrait, PlaybookCommandOutput};

#[derive(Debug, Deserialize, Serialize)]
pub enum AzureTasks {
    #[serde(rename = "dx.azure.login")]
    AzureLoginTask(AzureLoginTask),

    #[serde(rename = "dx.azure.cli")]
    AzureCliTask(AzureCliTask)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AzureLoginVars {
    pub client_id: String,
    pub secret: String,
    pub tenant: String
}

pub type AzureLoginTask = PlaybookCommand<AzureLoginVars>;

impl PlaybookCommandTrait for AzureLoginTask {
    fn execute(&mut self) {
        self.output = PlaybookCommandOutput::new();
        self.output.set_start_time();
        // add your code here
        self.output.stdout = format!("Running task: {}", self.command);
        self.output.stderr = format!("Error while running task: {}", self.command);  
        self.output.message = "Success".to_string();
        self.output.status = 1;
        self.output.success = 1;
        self.output.failed = 0;
        self.output.skipped = 0;
        self.output.changed = 0;

        self.output.set_end_time();

    }

    fn display(&self) {
        println!("Azure Login Task");
        println!("\tSelf: {:?}", self);
        println!("\tCommand: {}", self.command);
        println!("\tName: {}", self.name.as_ref().unwrap_or(&"No name".to_string()));        
        println!("\tOutput: {:?}", self.output);
    }

    fn output(&self) -> PlaybookCommandOutput {
        self.output.clone()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AzureCliVars {
    pub cmd: String
}

pub type AzureCliTask = PlaybookCommand<AzureCliVars>;

impl PlaybookCommandTrait for AzureCliTask {
    fn execute(&mut self) {
        self.output = PlaybookCommandOutput::new();
        self.output.set_start_time();

        let bash = AzCli::new(self.command.as_str());
        let output = bash.execute().expect("Failed to execute command");

        self.output.stdout = String::from_utf8_lossy(&output.stdout).to_string();
        self.output.stderr = String::from_utf8_lossy(&output.stderr).to_string();  
        self.output.message = "Success".to_string();
        self.output.status = 1;
        self.output.success = 1;
        self.output.failed = 0;
        self.output.skipped = 0;
        self.output.changed = 0;

        self.output.set_end_time();
    }

    fn display(&self) {
        println!("Azure Cli Task");
        println!("\tSelf: {:?}", self);
        println!("\tCommand: {}", self.command);
        println!("\tName: {}", self.name.as_ref().unwrap_or(&"No name".to_string()));        
        println!("\tOutput: {:?}", self.output);
    }

    fn output(&self) -> PlaybookCommandOutput {
        self.output.clone()
    }
}



impl PlaybookCommandTrait for AzureTasks {
    fn execute(&mut self) {
        match self {
            AzureTasks::AzureLoginTask(task) => task.execute(),
            AzureTasks::AzureCliTask(task) => task.execute(),
        }
    }

    fn display(&self) {
        match self {
            AzureTasks::AzureLoginTask(task) => task.display(),
            AzureTasks::AzureCliTask(task) => task.display(),
        }
    }

    fn output(&self) -> PlaybookCommandOutput {
        match self {
            AzureTasks::AzureLoginTask(task) => task.output(),
            AzureTasks::AzureCliTask(task) => task.output(),
        }
    }
}