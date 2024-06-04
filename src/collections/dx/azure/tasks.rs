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

    fn display(&self, verbose: Option<String>) {
        let verbose = verbose.unwrap_or("".to_string());
        println!("*** {} ***", self.name.as_ref().unwrap_or(&self.command));
        if verbose == "v" {
            println!("Task: {:?}", self);
            println!("Command: {}", self.command);
            println!("   === Output ===");
        }
        if verbose == "vv" {
            println!("{:?}", self.output);
        }
        else {
            println!("   === Output ===");
            println!("{}", self.output.stdout);
            println!("   === Errors ===");
            println!("{}", self.output.stderr);
        }
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

    fn display(&self, verbose: Option<String>) {
        let verbose = verbose.unwrap_or("".to_string());
        println!("*** {} ***", self.name.as_ref().unwrap_or(&self.command));
        if verbose == "v" {
            println!("Task: {:?}", self);
            println!("Command: {}", self.command);
            println!("   === Output ===");
        }
        if verbose == "vv" {
            println!("{:?}", self.output);
        }
        else {
            println!("   === Output ===");
            println!("{}", self.output.stdout);
            println!("   === Errors ===");
            println!("{}", self.output.stderr);
        }
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

    fn display(&self, verbose: Option<String>) {
        match self {
            AzureTasks::AzureLoginTask(task) => task.display(verbose),
            AzureTasks::AzureCliTask(task) => task.display(verbose),
        }
    }

    fn output(&self) -> PlaybookCommandOutput {
        match self {
            AzureTasks::AzureLoginTask(task) => task.output(),
            AzureTasks::AzureCliTask(task) => task.output(),
        }
    }
}