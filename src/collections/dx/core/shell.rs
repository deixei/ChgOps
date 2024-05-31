// class that represents a command line call to the sh shell

pub const BASH_COMMAND: &str = "sh";
pub const CMD_COMMAND: &str = "cmd";

use std::process::{Command, Output};
pub trait ShellTrait {
    fn execute(&self) -> Result<Output, std::io::Error>;
    fn display(&self, output: Output);
}

pub struct Shell {
    shell: String,
    command: String,
}

impl Shell {
    pub fn new(shell: &str, command: &str) -> Shell {
        Shell {
            shell: shell.to_string(),
            command: command.to_string(),
        }
    }
}

impl ShellTrait for Shell {
    fn execute(&self) -> Result<Output, std::io::Error> {
        Command::new(&self.shell)
            .arg("-c")
            .arg(&self.command)
            .output()
    }

    fn display(&self, output: Output) {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Command: {}\nOutput: {}\nErrors: {}", self.command, stdout, stderr);
    }
}

pub struct Bash {
    shell: Shell,
}

impl Bash {
    pub fn new(command: &str) -> Bash {
        Bash {
            shell: Shell::new(BASH_COMMAND, command),
        }
    }
}

impl ShellTrait for Bash {
    fn execute(&self) -> Result<Output, std::io::Error> {
        self.shell.execute()
    }

    fn display(&self, output: Output) {
        self.shell.display(output)
    }
}


pub struct WinCmd {
    shell: Shell,
}

impl WinCmd {
    pub fn new(command: &str) -> WinCmd {
        WinCmd {
            shell: Shell::new(CMD_COMMAND, command),
        }
    }
}

impl ShellTrait for WinCmd {
    fn execute(&self) -> Result<Output, std::io::Error> {
        self.shell.execute()
    }

    fn display(&self, output: Output) {
        self.shell.display(output)
    }
}