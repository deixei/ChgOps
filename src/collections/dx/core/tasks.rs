use serde::{Deserialize, Serialize};
use crate::collections::dx::core::shell::Bash;
use crate::collections::dx::core::shell::WinCmd;
use crate::collections::dx::core::shell::ShellTrait;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Tasks {
    BashCommandTask {
        name: String,
        #[serde(rename = "dx.core.bash")]
        command: String,
        register: Option<String>,
    },
    WinCmdCommandTask {
        name: String,
        #[serde(rename = "dx.core.wincmd")]
        command: String,
        register: Option<String>,
    },
}

pub fn execute(task) {

    match task {
        Tasks::BashCommandTask { name, command, .. } => {
            println!("Running task: {}", name);
            let bash = Bash::new(&command);
            let output = bash.execute().expect("Failed to execute command");
            bash.display(output);
        }
        Tasks::WinCmdCommandTask { name, command, .. } => {
            println!("Running task: {}", name);
            let wincmd = WinCmd::new(&command);
            let output = wincmd.execute().expect("Failed to execute command");
            wincmd.display(output);
        }
        
    }
}