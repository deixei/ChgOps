use serde::{Deserialize, Serialize};
pub mod core;
pub mod azure;
use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct PlaybookSummary {
    pub tasks_counter: i32,
    pub success_counter: i32,
    pub failed_counter: i32,
    pub skipped_counter: i32,
    pub changed_counter: i32,

    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

impl PlaybookSummary {
    pub fn new() -> PlaybookSummary {
        PlaybookSummary {
            tasks_counter: 0,
            success_counter: 0,
            failed_counter: 0,
            skipped_counter: 0,
            changed_counter: 0,
            start_time: None,
            end_time: None,
        }
    }

    pub fn increment_tasks(&mut self, tasks: i32) {
        self.tasks_counter += tasks;
    }

    pub fn increment_success(&mut self, success: i32) {
        self.success_counter += success;
    }

    pub fn increment_failed(&mut self, failed: i32) {
        self.failed_counter += failed;
    }

    pub fn increment_skipped(&mut self, skipped: i32) {
        self.skipped_counter += skipped;
    }

    pub fn increment_changed(&mut self, changed: i32) {
        self.changed_counter += changed;
    }

    pub fn increment_as_task(&mut self, output: PlaybookCommandOutput) {
        self.increment_tasks(1);
        self.increment_success(output.success);
        self.increment_failed(output.failed);
        self.increment_skipped(output.skipped);
        self.increment_changed(output.changed);
    }

    pub fn set_start_time(&mut self) {
        self.start_time = Some(Utc::now());
    }

    pub fn set_end_time(&mut self) {
        self.end_time = Some(Utc::now());
    }

    pub fn duration(&self) -> String {
        let duration1 = self.end_time.unwrap().signed_duration_since(self.start_time.unwrap());
        // set duration to a humam readable format
        let duration = HumanTime::from(duration1);

        duration.to_string()
    }

    pub fn display(&self) {
        println!("####### Playbook execution summary ##########");
        print!("Summary:\n\texecuted: {}", self.tasks_counter);
        print!("\tsuccess : {}", self.success_counter);
        print!("\tfailed  : {}", self.failed_counter);
        print!("\tskipped : {}", self.skipped_counter);
        println!("\tchanged : {}", self.changed_counter);
        
        let start_time_formatted = self.start_time.unwrap().format("%Y-%m-%d %H:%M:%S").to_string();
        let end_time_formatted = self.end_time.unwrap().format("%Y-%m-%d %H:%M:%S").to_string();

        print!("\tstart   : {:?}", start_time_formatted);
        print!("\tend     : {:?}", end_time_formatted);
        println!("\tduration: {:?}", self.duration());
        println!("#############################################");        
    }
}

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct Settings {
    pub name: String
}

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct Playbook {
    pub name: String,
    pub settings: Settings,

    pub tasks: Vec<PlaybookTasks>,

    #[serde(skip_deserializing)]
    engine_parameters: EngineParameters,
    
    #[serde(skip_deserializing)]
    summary: PlaybookSummary,
}

impl Playbook {
    pub fn new(name: &str, settings: Settings, tasks: Vec<PlaybookTasks>) -> Playbook {
        Playbook {
            name: name.to_string(),
            settings: settings,
            tasks: tasks,
            engine_parameters: EngineParameters::new("", "", "", ""),
            summary: PlaybookSummary::new(),
        }
    }

    pub fn set_engine_parameters(&mut self, engine_parameters: EngineParameters) {
        self.engine_parameters = engine_parameters;
    }

    pub fn display(&self) {
        println!("Playbook: {} #####################################", self.name);
        println!("\tSettings: {:?}", self.settings);
        println!("\tTasks: {:?}", self.tasks);
        self.engine_parameters.display();
        println!("#############################################");
    }

    pub fn display_summary(&self) {
        self.summary.display();
    }

    pub fn start_play(&mut self) {
        self.summary.set_start_time();
    }

    pub fn run_tasks(&mut self) {

        for task in self.tasks.iter_mut() {
            task.execute();
            task.display();
        }

    }

    pub fn end_play(&mut self) {
        self.summary.set_end_time();
        for task in self.tasks.iter() {
            let output = task.output();
            self.summary.increment_as_task(output);
        }
    }

}

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct EngineParameters {
    pub playbook_name: String,
    pub workspace_path: String,
    pub verbose: String,
    pub arguments: String,
}

impl EngineParameters {
    pub fn new(playbook_name: &str, workspace_path: &str, verbose: &str, arguments: &str) -> EngineParameters {
        EngineParameters {
            playbook_name: playbook_name.to_string(),
            workspace_path: workspace_path.to_string(),
            verbose: verbose.to_string(),
            arguments: arguments.to_string(),
        }
    }

    pub fn display(&self) {
        println!("Engine Parameters ###########################");

        println!("\tPlaybook Name: {}", self.playbook_name);
        println!("\tWorkspace Path: {}", self.workspace_path);
        println!("\tVerbose: {}", self.verbose);
        println!("\tArguments: {}", self.arguments);
        println!("#############################################");
    }
}

#[derive(Debug, Deserialize, Default, Serialize, Clone)]
pub struct PlaybookCommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub message: String,
    pub status: i32,
    pub success: i32,
    pub failed: i32,
    pub skipped: i32,
    pub changed: i32,

    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

impl PlaybookCommandOutput {
    pub fn new() -> PlaybookCommandOutput {
        PlaybookCommandOutput {
            stdout: "".to_string(),
            stderr: "".to_string(),
            message: "".to_string(),
            status: 0,
            success: 0,
            failed: 0,
            skipped: 0,
            changed: 0,
            start_time: None,
            end_time: None,
        }
    }

    pub fn set_start_time(&mut self) {
        self.start_time = Some(Utc::now());
    }

    pub fn set_end_time(&mut self) {
        self.end_time = Some(Utc::now());
    }

    pub fn display(&self) {
        println!("####### Playbook Command Output ##########");
        println!("\tstdout: {:?}", self.stdout);
        println!("\tstderr: {:?}", self.stderr);
        println!("\tmessage: {:?}", self.message);
        println!("\tstatus: {:?}", self.status);
        println!("\tsuccess: {:?}", self.success);
        println!("\tfailed: {:?}", self.failed);
        println!("\tskipped: {:?}", self.skipped);
        println!("\tchanged: {:?}", self.changed);

        let start_time_formatted = self.start_time.unwrap().format("%Y-%m-%d %H:%M:%S").to_string();
        let end_time_formatted = self.end_time.unwrap().format("%Y-%m-%d %H:%M:%S").to_string();

        print!("\tstart   : {:?}", start_time_formatted);
        print!("\tend     : {:?}", end_time_formatted);
        println!("\tduration: {:?}", self.duration());
        println!("#############################################");        
    }
    pub fn duration(&self) -> String {
        let duration1 = self.end_time.unwrap().signed_duration_since(self.start_time.unwrap());
        // set duration to a humam readable format
        let duration = HumanTime::from(duration1);

        duration.to_string()
    }
}

pub trait PlaybookCommandTrait {
    fn execute(&mut self);
    fn display(&self);
    fn output(&self) -> PlaybookCommandOutput;
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PlaybookCommand<T> {
    pub command: String,
    pub name: Option<String>,
    pub vars: Option<T>,
    pub register: Option<String>,
    pub state: Option<String>,
    pub when: Option<String>,

    #[serde(skip_deserializing)]
    pub output: PlaybookCommandOutput,

    #[serde(skip_deserializing)]
    pub parent: Playbook,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PlaybookTasks {
    CoreTasks(crate::collections::dx::core::tasks::CoreTasks),
    AzureTasks(crate::collections::dx::azure::tasks::AzureTasks),
}


impl PlaybookCommandTrait for PlaybookTasks {
    fn execute(&mut self) {
        match self {
            PlaybookTasks::CoreTasks(task) => task.execute(),
            PlaybookTasks::AzureTasks(task) => task.execute(),
        }
    }

    fn display(&self) {
        match self {
            PlaybookTasks::CoreTasks(task) => task.display(),
            PlaybookTasks::AzureTasks(task) => task.display(),
        }
    }

    fn output(&self) -> PlaybookCommandOutput {
        match self {
            PlaybookTasks::CoreTasks(task) => task.output(),
            PlaybookTasks::AzureTasks(task) => task.output(),
        }
    }
}