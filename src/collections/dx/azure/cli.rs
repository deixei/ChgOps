// Module: cli
use std::process::{Command, Output};

pub struct AzCli {
    command: String,
}

impl AzCli {
    pub fn new(command: &str) -> AzCli {
        AzCli {
            command: command.to_string(),
        }
    }
    pub fn execute(&self) -> Result<Output, std::io::Error> {
        Command::new("az")
            .arg(&self.command)
            .output()
    }

    pub fn display(&self, output: Output) {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Command: {}\nOutput: {}\nErrors: {}", self.command, stdout, stderr);
    }
}