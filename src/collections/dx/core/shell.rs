// class that represents a command line call to the sh shell

const BASH_COMMAND: &str = "sh";

use std::process::{Command, Output};

pub struct Bash {
    command: String,
}

impl Bash {
    pub fn new(command: &str) -> Bash {
        Bash {
            command: command.to_string(),
        }
    }

    pub fn execute(&self) -> Result<Output, std::io::Error> {
        Command::new(&BASH_COMMAND)
            .arg("-c")
            .arg(&self.command)
            .output()
    }

    pub fn display(&self, output: Output) {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Command: {}\nOutput: {}\nErrors: {}", self.command, stdout, stderr);
    }
}