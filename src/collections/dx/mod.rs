use serde::{Deserialize, Serialize};
pub mod core;
pub mod azure;
use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::env;
use serde_yaml;
use std::collections::HashMap;
use yaml_rust2::{YamlLoader, Yaml, YamlEmitter};
use std::fs::File;
use std::io::prelude::*;

use tera::{Tera, Context};
use serde_json::Value;

use crate::collections::dx::core::filters;


pub fn open_yaml(filename: &str) -> Vec<Yaml> {
    let mut f = File::open(filename).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let docs = YamlLoader::load_from_str(&s).unwrap();

    docs
}

pub fn print_indent(indent: usize) {
    for _ in 0..indent {
        print!("    ");
    }
}

pub fn dump_yaml(doc: &Yaml, indent: usize) {
    match *doc {
        Yaml::Array(ref v) => {
            for x in v {
                dump_yaml(x, indent + 1);
            }
        }
        Yaml::Hash(ref h) => {
            for (k, v) in h {
                print_indent(indent);
                println!("{k:?}:");
                dump_yaml(v, indent + 1);
            }
        }
        _ => {
            print_indent(indent);
            println!("{doc:?}");
        }
    }
}

fn merge_yaml(yaml1: &mut Yaml, yaml2: &Yaml) {
    match (yaml1, yaml2) {
        (Yaml::Hash(hash1), Yaml::Hash(hash2)) => {
            for (key, value) in hash2 {
                merge_yaml(hash1.entry(key.clone()).or_insert(Yaml::Null), value);
            }
        },
        (target, source) => {
            *target = source.clone();
        },
    }
}

#[derive(Debug, Default)]
pub struct ChgOpsWorkspace {
    pub current_dir: String,
    pub workspace_path: String,

    // engine_parameters
    pub playbook_name: String,
    pub verbose: String,
    pub arguments: String,

    pub playbook: Playbook,
    pub configurations: Vec<Yaml>,
    pub variables: Vec<Yaml>,
    pub summary: PlaybookSummary,


    pub active_playbook_document: String,


}

impl ChgOpsWorkspace {

    pub fn new() -> ChgOpsWorkspace {
        let current_dir = env::current_dir().unwrap().to_str().unwrap().to_owned();
        ChgOpsWorkspace {
            current_dir: current_dir.clone(),
            workspace_path: current_dir.clone(),
            playbook_name: "".to_string(),
            verbose: "".to_string(),
            arguments: "".to_string(),

            playbook: Playbook::new("",
                Settings::default(),
                vec![]),
            configurations: vec![],
            variables: vec![],
            summary: PlaybookSummary::new(),
            active_playbook_document: "".to_string(),
        }
    }

    pub fn workspace_path(&mut self) -> String {
        let workspace_path = if self.workspace_path.is_empty() {
            self.current_dir.to_string()
        } else {
            self.workspace_path.to_string()
        };

        workspace_path
    }

    pub fn playbook_full_path(&mut self) -> String {
        format!("{}/{}.yaml", &self.workspace_path(), &self.playbook_name)
    }

    pub fn vars_full_path(&mut self) -> String {
        format!("{}/vars.yaml", &self.workspace_path())
    }

    pub fn config_full_path(&mut self) -> String {
        format!("{}/config.yaml", &self.workspace_path())
    }

    pub fn load_workspace(&mut self) {
        let playbook_full_path = self.playbook_full_path();

        self.configurations = open_yaml(&self.config_full_path());
        self.variables = open_yaml(&self.vars_full_path());
        let doc1 = &mut self.configurations[0];

        let doc2 = &self.variables[0];
        
        let playbook_yaml_data = open_yaml(&playbook_full_path);
        let doc3  = &playbook_yaml_data[0];

        merge_yaml(doc1, doc2);
        merge_yaml(doc1, doc3);
        

        // show the merged yaml
        // dump_yaml(doc1, 0);
        

        // Convert the merged YAML documents to a String
        let mut out_str = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.dump(doc1).unwrap(); // Dump the YAML object to a String
        }
        
        // Convert the YAML document to a JSON Value
        let json_value: Value = serde_yaml::from_str(&out_str).unwrap();

        // at this point we should not have any templates to be replaced, so we can convert the JSON Value to a Context
        // check if there are any templates to be replaced
        

        // Convert the JSON Value to a Context
        let context = Context::from_value(json_value).unwrap();
        //println!("{:#?}", context);

        let mut tera = Tera::default();
        let _ = tera.add_raw_template("hello", &out_str);
        tera.register_function("current_time", filters::current_time());

        tera.register_function("env_var", filters::env_var());
        //tera.register_function("url_for", filters::make_url_for("demo".to_string()));

        tera.register_filter("filter1", filters::filter1);
        tera.register_filter("filter2", filters::filter2);
        
        // Render a template with the context
        let rendered = tera.render("hello", &context).unwrap();
        //println!("{}", rendered);

        let _ = tera.add_raw_template("postprocess", &rendered);

        self.active_playbook_document = tera.render("postprocess", &context).unwrap();

        println!("{}", &self.active_playbook_document);

        self.playbook = serde_yaml::from_str(&self.active_playbook_document)
            .expect("Failed to parse playbook");
    }

    pub fn run_playbook(&mut self) {
        self.summary.set_start_time();
        self.start_banner();

        self.playbook.run_tasks(Some(self.verbose.clone()));
        
        // generate summary
        for task in self.playbook.tasks.iter() {
            let output = task.output();
            self.summary.increment_as_task(output);
        }

        self.end_banner();
    }

    pub fn start_banner(&mut self) {
        println!("ChgOps - Change management and operations tool");

        println!("Engine Parameters ###########################");

        println!("\tPlaybook Name: {}", &self.playbook_name);
        println!("\tWorkspace Path: {}", self.workspace_path());
        println!("\tVerbose: {}", &self.verbose);
        println!("\tArguments: {}", &self.arguments);
        println!("\tFiles information:");
        println!("\t\tCurrent Dir: {}", &self.current_dir);
        println!("\t\tPlaybook Full Path: {}", self.playbook_full_path());
        println!("\t\tConfigurations Full Path: {}", self.config_full_path());
        println!("\t\tVariables Full Path: {}", self.vars_full_path());
        println!("#############################################");

        self.playbook.display(Some(self.verbose.clone()));
    }

    pub fn end_banner(&mut self) {
        self.summary.set_end_time();

        self.summary.display();
        println!("ChgOps - End of execution");
    }

    pub fn display(&mut self) {
        println!("Workspace Facts ###########################");
        println!("\tWorkspace Path: {}", &self.workspace_path());
        println!("#############################################");
    }
}

lazy_static! {
    pub static ref WORKSPACE: Mutex<ChgOpsWorkspace> = Mutex::new(ChgOpsWorkspace::new());
}



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
pub struct WorkspaceConfigurations {
    pub params: Option<String>
}

impl WorkspaceConfigurations {
    pub fn new() -> WorkspaceConfigurations {
        WorkspaceConfigurations {
            params: None
        }
    }
}


#[derive(Debug, Deserialize, Default, Serialize)]
pub struct WorkspaceVariables {
    pub params: Option<HashMap<String, String>>,
    pub vars: Option<HashMap<String, String>>,
}

impl WorkspaceVariables {
    pub fn new() -> WorkspaceVariables {
        WorkspaceVariables {
            params: None,
            vars: None,
        }
    }
}

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct Settings {
    pub name: String,
    pub vars: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct Playbook {
    pub name: String,
    pub settings: Settings,

    pub tasks: Vec<PlaybookTasks>,

}

impl Playbook {
    pub fn new(name: &str, settings: Settings, tasks: Vec<PlaybookTasks>) -> Playbook {
        Playbook {
            name: name.to_string(),
            settings: settings,
            tasks: tasks
        }
    }

    pub fn display(&self, verbose: Option<String>) {
        let verbose = verbose.unwrap_or("".to_string());
        println!("Playbook: {} #####################################", self.name);
        if verbose.len() >= 1  {
            println!("\tSettings: {:?}", self.settings);
        }
        if verbose.len() >= 2 {
            println!("\tTasks: {:?}", self.tasks);
        }
        println!("\tTasks count: {:?}", self.tasks.len());
        println!("#############################################");
    }

    pub fn run_tasks(&mut self, verbose: Option<String>) {

        for task in self.tasks.iter_mut() {
            task.execute();
            task.display(verbose.clone());
        }

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
    fn display(&self, verbose: Option<String>);
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

    fn display(&self, verbose: Option<String>) {
        match self {
            PlaybookTasks::CoreTasks(task) => task.display(verbose),
            PlaybookTasks::AzureTasks(task) => task.display(verbose),
        }
    }

    fn output(&self) -> PlaybookCommandOutput {
        match self {
            PlaybookTasks::CoreTasks(task) => task.output(),
            PlaybookTasks::AzureTasks(task) => task.output(),
        }
    }
}