use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Tasks {
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