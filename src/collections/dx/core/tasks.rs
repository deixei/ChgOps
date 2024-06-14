use serde::{Deserialize, Serialize};
use serde_yaml::Value as YamlValue;
use crate::collections::dx::core::shell::Bash;
use crate::collections::dx::core::shell::WinCmd;
use crate::collections::dx::core::shell::ShellTrait;
use crate::collections::dx::{PlaybookCommand, PlaybookCommandTrait, PlaybookCommandOutput};

#[derive(Debug, Deserialize, Serialize)]
pub enum CoreTasks {
    #[serde(rename = "dx.core.bash")]
    BashCommandTask(BashCommandTask),

    #[serde(rename = "dx.core.wincmd")]
    WinCmdCommandTask(WinCmdCommandTask),

    #[serde(rename = "dx.core.print")]
    PrintCommandTask(PrintCommandTask),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BashCommandVars {
    pub resource: YamlValue,
}

pub type BashCommandTask = PlaybookCommand<String, BashCommandVars>;

impl PlaybookCommandTrait for BashCommandTask {
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
                
        let bash = Bash::new(&self.command);
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
        println!("*** {} *** [e:{}/s:{}/f:{}/s:{}/c:{}] ***", 
            self.name.as_ref().unwrap_or(&self.command),
            self.output.status,
            self.output.success,
            self.output.failed,
            self.output.skipped,
            self.output.changed
        );
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
pub struct WinCmdCommandVars {
    pub resource: YamlValue,
}

pub type WinCmdCommandTask = PlaybookCommand<String, WinCmdCommandVars>;

impl PlaybookCommandTrait for WinCmdCommandTask {
    fn execute(&mut self) {
        self.output = PlaybookCommandOutput::new();
        self.output.set_start_time();

        // eval it self.when is not empty and the when is true
        let when = self.when.clone().unwrap_or("true".to_string());

        if when == "false"{
            self.output.message = "Skipped".to_string();
            self.output.skipped = 1;
            self.output.set_end_time();
            return;
        }

        let wincmd = WinCmd::new(&self.command);
        let output = wincmd.execute().expect("Failed to execute command");


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
        println!("*** {} *** [e:{}/s:{}/f:{}/s:{}/c:{}] ***", 
            self.name.as_ref().unwrap_or(&self.command),
            self.output.status,
            self.output.success,
            self.output.failed,
            self.output.skipped,
            self.output.changed
        );
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

pub type PrintCommandTask = PlaybookCommand<YamlValue, PrintCommandVars>;

#[derive(Debug, Deserialize, Serialize)]
pub struct PrintCommandVars {
    pub resource: YamlValue,
}

impl PlaybookCommandTrait for PrintCommandTask {
    fn execute(&mut self) {
        self.output = PlaybookCommandOutput::new();
        self.output.set_start_time();

        let when = self.when.clone().unwrap_or("true".to_string());
        let register = self.register.clone().unwrap_or("".to_string());
        let state = self.state.clone().unwrap_or("present".to_string());


        if when == "false"{
            self.output.message = "Skipped".to_string();
            self.output.skipped = 1;
            self.output.set_end_time();
            return;
        }

        self.output.stdout = serde_yaml::to_string(&self.command).unwrap();

        self.output.stderr = "".to_string();
        
        self.output.message = "Success".to_string();
        if state == "absent" {
            self.output.message = "Removed".to_string();
        }
        if state == "present" {
            self.output.message = "Success".to_string();
        }

        
        self.output.status = 1;
        self.output.success = 0;
        self.output.failed = 0;
        self.output.skipped = 0;
        self.output.changed = 0;

        if register != "" {
            // add to the central fact store this reference
            //let mut workspace = WORKSPACE.lock().unwrap();
            //workspace.facts.insert(register.clone(), self.output.clone());
        }

        self.output.set_end_time();
    }

    fn display(&self, verbose: Option<String>) {
        let verbose = verbose.unwrap_or("".to_string());

        let command_str = serde_yaml::to_string(&self.command).unwrap();

        println!("*** {} *** [e:{}/s:{}/f:{}/s:{}/c:{}] ***", 
            self.name.as_ref().unwrap_or(&command_str),
            self.output.status,
            self.output.success,
            self.output.failed,
            self.output.skipped,
            self.output.changed
        );
        if verbose == "v" {
            println!("Task: {:?}", self);
            println!("Command: {}", command_str);
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




impl PlaybookCommandTrait for CoreTasks {
    fn execute(&mut self) {
        match self {
            CoreTasks::BashCommandTask(task) => task.execute(),
            CoreTasks::WinCmdCommandTask(task) => task.execute(),
            CoreTasks::PrintCommandTask(task) => task.execute(),
        }
    }

    fn display(&self, verbose: Option<String>) {
        match self {
            CoreTasks::BashCommandTask(task) => task.display(verbose),
            CoreTasks::WinCmdCommandTask(task) => task.display(verbose),
            CoreTasks::PrintCommandTask(task) => task.display(verbose),
        }
    }

    fn output(&self) -> PlaybookCommandOutput {
        match self {
            CoreTasks::BashCommandTask(task) => task.output(),
            CoreTasks::WinCmdCommandTask(task) => task.output(),
            CoreTasks::PrintCommandTask(task) => task.output(),
        }
    }
}