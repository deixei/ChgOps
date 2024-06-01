use serde::{Deserialize, Serialize};
use crate::collections::dx::core::shell::Bash;
use crate::collections::dx::core::shell::WinCmd;
use crate::collections::dx::core::shell::ShellTrait;
use crate::collections::dx::{PlaybookCommand, PlaybookCommandTrait, PlaybookCommandOutput, OkPlaybookCommandOutput, ErrPlaybookCommandOutput};


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
    fn execute(&self) -> Result<OkPlaybookCommandOutput, ErrPlaybookCommandOutput> {

        println!("BashCommandTask -- Running task: {:?}", self.command);
        let bash = Bash::new(&self.command);
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

pub type WinCmdCommandTask = PlaybookCommand<WinCmdCommandVars>;
impl PlaybookCommandTrait for WinCmdCommandTask {
    fn execute(&self) -> Result<OkPlaybookCommandOutput, ErrPlaybookCommandOutput> {
        println!("WinCmdCommandTask -- Running task: {:?}", self.command);
        let wincmd = WinCmd::new(&self.command);
        let output = wincmd.execute().expect("Failed to execute command");
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

impl PlaybookCommandTrait for CoreTasks {
    fn execute(&self) -> Result<OkPlaybookCommandOutput, ErrPlaybookCommandOutput> {
        match self {
            CoreTasks::BashCommandTask(task) => task.execute(),
            CoreTasks::WinCmdCommandTask(task) => task.execute(),
        }
    }

    fn display(&self, output: PlaybookCommandOutput) {
        match self {
            CoreTasks::BashCommandTask(task) => task.display(output),
            CoreTasks::WinCmdCommandTask(task) => task.display(output),
        }
    }
}