// Module: cli
use std::process::{Command, Output};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct AzCli {
    pub command: String,
}

impl AzCli {
    pub fn new(command: &str) -> AzCli {
        AzCli {
            command: command.to_string(),
        }
    }
    pub fn execute(&self) -> Result<Output, std::io::Error> {
        // if this is a windows machine, we need to use cmd /c az   
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg("/c")
                .arg("az")
                .arg(&self.command)
                .output()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("az")
                .arg(&self.command)
                .output()
        }
    }

    pub fn display(&self, output: Output) {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Command: {}\nOutput: {}\nErrors: {}", self.command, stdout, stderr);
    }
}


// based on az login --service-principal -u $AZURE_CLIENT_ID -p $AZURE_SECRET --tenant $AZURE_TENANT make a new impl for this that takes in the client_id, secret, and tenant, and if missing read them from env variables
#[derive(Debug, Deserialize, Serialize)]
pub struct AzLogin {
    client_id: String,
    secret: String,
    tenant: String,
}

impl AzLogin {
    pub fn new(mut client_id: String, mut secret: String, mut tenant: String) -> AzLogin {
        if client_id.is_empty() {
            client_id = std::env::var("AZURE_CLIENT_ID").expect("AZURE_CLIENT_ID is not set");
        }
        if secret.is_empty() {
            secret = std::env::var("AZURE_SECRET").expect("AZURE_SECRET is not set");
        }
        if tenant.is_empty() {
            tenant = std::env::var("AZURE_TENANT").expect("AZURE_TENANT is not set");
        }
    
        AzLogin {
            client_id,
            secret,
            tenant,
        }
    }
    pub fn execute(&self) -> Result<Output, std::io::Error> {
        // if this is a windows machine, we need to use cmd /c az   
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg("/c")
                .arg("az")
                .arg("login")
                .arg("--service-principal")
                .arg("-u")
                .arg(&self.client_id)
                .arg("-p")
                .arg(&self.secret)
                .arg("--tenant")
                .arg(&self.tenant)
                .output()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("az")
                .arg("login")
                .arg("--service-principal")
                .arg("-u")
                .arg(&self.client_id)
                .arg("-p")
                .arg(&self.secret)
                .arg("--tenant")
                .arg(&self.tenant)
                .output()
        }
    }

    pub fn display(&self, output: Output) {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Command: az login --service-principal -u {} -p {} --tenant {}\nOutput: {}\nErrors: {}", self.client_id, self.secret, self.tenant, stdout, stderr);
    }
}