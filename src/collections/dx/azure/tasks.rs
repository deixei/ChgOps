use serde::{Deserialize, Serialize};
use crate::collections::dx::{azure::cli::AzCli, PlaybookCommand, PlaybookCommandTrait, PlaybookCommandOutput, OkPlaybookCommandOutput, ErrPlaybookCommandOutput};
use std::process::Output;

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
    fn execute(&self) -> Result<OkPlaybookCommandOutput, ErrPlaybookCommandOutput> {
        let success_message: String = format!("Running task: {}", self.command);
        let error_message: String = format!("Error while running task: {}", self.command);

        let output_result = PlaybookCommandOutput {
            stdout: success_message,
            stderr: error_message,
            message: "Success".to_string(),
            status: 0,
            success: 1,
            failed: 0,
            skipped: 0,
            changed: 0,
        };
        
        Ok(output_result)
    }

    fn display(&self, output: PlaybookCommandOutput) {
        println!("AzureLoginTask");
        println!("\tSelf: {:?}", self);
        println!("\tCommand: {}", self.command);
        println!("\tName: {}", self.name.as_ref().unwrap_or(&"No name".to_string()));        
        println!("\tOutput: {:?}", output);
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AzureCliVars {
    pub cmd: String
}

pub type AzureCliTask = PlaybookCommand<AzureCliVars>;

impl PlaybookCommandTrait for AzureCliTask {
    fn execute(&self) -> Result<OkPlaybookCommandOutput, ErrPlaybookCommandOutput> {
        let success_message: String = format!("Running task: {}", self.command);
        let error_message: String = format!("Error while running task: {}", self.command);

        let bash = AzCli::new(self.command.as_str());
        let output = bash.execute().expect("Failed to execute command");

        let stdout:String = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr:String = String::from_utf8_lossy(&output.stderr).to_string();

        let output_result = PlaybookCommandOutput {
            stdout: stdout,
            stderr: stderr,
            message: "Success".to_string(),
            status: 0,
            success: 1,
            failed: 0,
            skipped: 0,
            changed: 0,
        };
        
        Ok(output_result)
    }

    fn display(&self, output: PlaybookCommandOutput) {
        println!("AzureCliTask");
        println!("\tSelf: {:?}", self);
        println!("\tCommand: {}", self.command);
        println!("\tName: {}", self.name.as_ref().unwrap_or(&"No name".to_string()));        
        println!("\tOutput: {:?}", output);
    }
}



impl PlaybookCommandTrait for AzureTasks {
    fn execute(&self) -> Result<OkPlaybookCommandOutput, ErrPlaybookCommandOutput> {
        match self {
            AzureTasks::AzureLoginTask(task) => task.execute(),
            AzureTasks::AzureCliTask(task) => task.execute(),
        }
    }

    fn display(&self, output: PlaybookCommandOutput) {
        match self {
            AzureTasks::AzureLoginTask(task) => task.display(output),
            AzureTasks::AzureCliTask(task) => task.display(output),
        }
    }
}