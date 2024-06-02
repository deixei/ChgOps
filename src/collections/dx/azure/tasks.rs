use serde::{Deserialize, Serialize};
use crate::collections::dx::{azure::cli::AzCli, PlaybookCommand, PlaybookCommandTrait, PlaybookCommandOutput};
use crate::collections::dx::Playbook;

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
    fn execute(&mut self, playbook: &mut Playbook) {


        self.output = PlaybookCommandOutput::new();

        self.output.stdout = format!("Running task: {}", self.command);
        self.output.stderr = format!("Error while running task: {}", self.command);  
        self.output.message = "Success".to_string();
        self.output.status = 1;
        self.output.success = 1;
        self.output.failed = 0;
        self.output.skipped = 0;
        self.output.changed = 0;



    }

    fn display(&self) {
        println!("AzureLoginTask");
        println!("\tSelf: {:?}", self);
        println!("\tCommand: {}", self.command);
        println!("\tName: {}", self.name.as_ref().unwrap_or(&"No name".to_string()));        
        println!("\tOutput: {:?}", self.output);
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AzureCliVars {
    pub cmd: String
}

pub type AzureCliTask = PlaybookCommand<AzureCliVars>;

impl PlaybookCommandTrait for AzureCliTask {
    fn execute(&mut self, playbook: &mut Playbook) {

        let bash = AzCli::new(self.command.as_str());
        let output = bash.execute().expect("Failed to execute command");

        self.output = PlaybookCommandOutput::new();

        self.output.stdout = String::from_utf8_lossy(&output.stdout).to_string();
        self.output.stderr = String::from_utf8_lossy(&output.stderr).to_string();  
        self.output.message = "Success".to_string();
        self.output.status = 1;
        self.output.success = 1;
        self.output.failed = 0;
        self.output.skipped = 0;
        self.output.changed = 0;


    }

    fn display(&self) {
        println!("AzureCliTask");
        println!("\tSelf: {:?}", self);
        println!("\tCommand: {}", self.command);
        println!("\tName: {}", self.name.as_ref().unwrap_or(&"No name".to_string()));        
        println!("\tOutput: {:?}", self.output);
    }
}



impl PlaybookCommandTrait for AzureTasks {
    fn execute(&mut self, playbook: &mut Playbook) {
        match self {
            AzureTasks::AzureLoginTask(task) => task.execute(playbook),
            AzureTasks::AzureCliTask(task) => task.execute(playbook),
        }
    }

    fn display(&self) {
        match self {
            AzureTasks::AzureLoginTask(task) => task.display(),
            AzureTasks::AzureCliTask(task) => task.display(),
        }
    }
}