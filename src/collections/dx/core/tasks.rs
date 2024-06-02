use serde::{Deserialize, Serialize};
use crate::collections::dx::core::shell::Bash;
use crate::collections::dx::core::shell::WinCmd;
use crate::collections::dx::core::shell::ShellTrait;
use crate::collections::dx::{PlaybookCommand, PlaybookCommandTrait, PlaybookCommandOutput};
use crate::collections::dx::Playbook;

#[derive(Debug, Deserialize, Serialize)]
pub enum CoreTasks {
    #[serde(rename = "dx.core.bash")]
    BashCommandTask(BashCommandTask),

    #[serde(rename = "dx.core.wincmd")]
    WinCmdCommandTask(WinCmdCommandTask)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BashCommandVars {
    pub cli: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WinCmdCommandVars {
    pub cli: String
}

pub type BashCommandTask = PlaybookCommand<BashCommandVars>;
impl PlaybookCommandTrait for BashCommandTask {
    fn execute(&mut self, playbook: &mut Playbook) {

        println!("BashCommandTask -- Running task: {:?}", self.command);
        let bash = Bash::new(&self.command);
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
        println!("\tOutput: {:?}", self. output);
    }
}

pub type WinCmdCommandTask = PlaybookCommand<WinCmdCommandVars>;
impl PlaybookCommandTrait for WinCmdCommandTask {
    fn execute(&mut self, playbook: &mut Playbook) {

        println!("WinCmdCommandTask -- Running task: {:?}", self.command);
        let wincmd = WinCmd::new(&self.command);
        let output = wincmd.execute().expect("Failed to execute command");

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

impl PlaybookCommandTrait for CoreTasks {
    fn execute(&mut self, playbook: &mut Playbook) {
        match self {
            CoreTasks::BashCommandTask(task) => task.execute(playbook),
            CoreTasks::WinCmdCommandTask(task) => task.execute(playbook),
        }
    }

    fn display(&self) {
        match self {
            CoreTasks::BashCommandTask(task) => task.display(),
            CoreTasks::WinCmdCommandTask(task) => task.display(),
        }
    }
}