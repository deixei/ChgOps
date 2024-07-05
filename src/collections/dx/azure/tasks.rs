use serde::{Deserialize, Serialize};
use crate::collections::dx::{azure::cli::AzCli, PlaybookCommand, PlaybookCommandTrait, PlaybookCommandOutput};
use serde_yaml::Value as YamlValue;
use crate::{print_error, print_warning, print_info, print_success, print_banner_yellow, print_banner_green, print_banner_red, print_banner_blue};

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

pub type AzureLoginTask = PlaybookCommand<AzureLoginVars, YamlValue>;

impl PlaybookCommandTrait for AzureLoginTask {
    fn execute(&mut self) {
        self.output = PlaybookCommandOutput::new();
        self.output.set_start_time();

        let when = self.when.clone().unwrap_or("true".to_string());

        if when == "false"{
            self.output.message = "Skipped".to_string();
            self.output.skipped = 1;
            self.output.set_end_time();
            return;
        }

        // add your code here
        self.output.stdout = format!("Running task: {:#?}", self.command);
        self.output.stderr = format!("Error while running task: {:#?}", self.command);  
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
        print_banner_blue!("TASK: *** {} *** [St.:{}/Succ.:{}/Fail:{}/Skip:{}/Chg:{}] ***", 
            self.name.as_ref().unwrap_or(&"Unnamed".to_string()),
            self.output.status,
            self.output.success,
            self.output.failed,
            self.output.skipped,
            self.output.changed
        );
        if verbose.len() >= 1 {
            print_info!("Task details: {:?}", self);
            //print_info!("Command: {}", command_str);
        }
        if verbose.len() >= 2 {
            print_banner_yellow!("=== Output Obj ===");
            print_info!("{:?}", self.output);
        }
        else {
            if self.output.stdout != "" {
                print_banner_green!("=== Output ===");
                print_success!("{}", self.output.stdout);
            }
            if self.output.stderr != "" {
                print_banner_red!("=== Errors ===");
                print_error!("{}", self.output.stderr);
            }
        }
    }

    fn output(&self) -> PlaybookCommandOutput {
        self.output.clone()
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct AzureCliVars {
    pub resource: YamlValue,
}

pub type AzureCliTask = PlaybookCommand<String, AzureCliVars>;

impl PlaybookCommandTrait for AzureCliTask {
    fn execute(&mut self) {
        self.output = PlaybookCommandOutput::new();
        self.output.set_start_time();

        let when = self.when.clone().unwrap_or("true".to_string());

        if when == "false"{
            self.output.message = "Skipped".to_string();
            self.output.skipped = 1;
            self.output.set_end_time();
            return;
        }

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
        print_banner_blue!("TASK: *** {} *** [St.:{}/Succ.:{}/Fail:{}/Skip:{}/Chg:{}] ***", 
            self.name.as_ref().unwrap_or(&"Unnamed".to_string()),
            self.output.status,
            self.output.success,
            self.output.failed,
            self.output.skipped,
            self.output.changed
        );
        if verbose.len() >= 1 {
            print_info!("Task details: {:?}", self);
            //print_info!("Command: {}", command_str);
        }
        if verbose.len() >= 2 {
            print_banner_yellow!("=== Output Obj ===");
            print_info!("{:?}", self.output);
        }
        else {
            if self.output.stdout != "" {
                print_banner_green!("=== Output ===");
                print_success!("{}", self.output.stdout);
            }
            if self.output.stderr != "" {
                print_banner_red!("=== Errors ===");
                print_error!("{}", self.output.stderr);
            }
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