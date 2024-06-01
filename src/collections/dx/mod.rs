pub mod core;
pub mod azure;

use serde::{Deserialize, Serialize};
use std::process::Output;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub name: String
}

#[derive(Debug, Deserialize)]
pub struct Playbook {
    pub name: String,
    pub settings: Settings,
    pub tasks: Vec<PlaybookTasks>,
}

#[derive(Debug, Deserialize)]
pub struct EngineParameters {
    pub playbook_name: String,
    pub workspace_path: String,
    pub verbose: String,
    pub arguments: String,
}

#[derive(Debug, Deserialize)]
pub struct PlaybookCommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub message: String,
    pub status: i32,
    pub success: i32,
    pub failed: i32,
    pub skipped: i32,
    pub changed: i32,
}

pub type OkPlaybookCommandOutput = PlaybookCommandOutput;
pub type ErrPlaybookCommandOutput = PlaybookCommandOutput;

pub trait PlaybookCommandTrait {
    fn execute(&self) -> Result<OkPlaybookCommandOutput, ErrPlaybookCommandOutput>;
    fn display(&self, output: PlaybookCommandOutput);
}

// Playbook Sample:
// tasks:
//   - dx.core.bash
//      command: "dx.core.bash"
//      register: "output"
//      vars:
//          var1: "value1"
//          var2: "value2"
//   - dx.azure.cli
//      command: "dx.azure.cli"
//      register: "output2"
//      state: "absent"
//   - block:
//       - name: "Task 3"
//         command: "echo 'Hello World 3'"

#[derive(Debug, Deserialize, Serialize)]
pub struct PlaybookCommand<T> {
    pub command: String,
    pub name: Option<String>,
    pub vars: Option<T>,
    pub register: Option<String>,
    pub state: Option<String>,
    pub when: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PlaybookTasks {
    CoreTasks(crate::collections::dx::core::tasks::CoreTasks),
    AzureTasks(crate::collections::dx::azure::tasks::AzureTasks),
}
