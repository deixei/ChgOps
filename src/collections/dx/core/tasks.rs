use serde::{Deserialize, Serialize};
use serde_yaml::Value as YamlValue;
use crate::collections::dx::config_proc;
use crate::collections::dx::core::shell::Bash;
use crate::collections::dx::core::shell::WinCmd;
use crate::collections::dx::core::shell::ShellTrait;
use crate::collections::dx::{PlaybookCommand, PlaybookCommandTrait, PlaybookCommandOutput};
use crate::collections::dx::FACTS;
use crate::{print_error, print_warning, print_info, print_success, print_banner_yellow, print_banner_green, print_banner_red, print_banner_blue};

// register task execution here:
#[derive(Debug, Deserialize, Serialize)]
pub enum CoreTasks {
    #[serde(rename = "dx.core.bash")]
    BashCommandTask(BashCommandTask),

    #[serde(rename = "dx.core.wincmd")]
    WinCmdCommandTask(WinCmdCommandTask),

    #[serde(rename = "dx.core.print")]
    PrintCommandTask(PrintCommandTask),
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct BashCommandVars {
    pub resource: YamlValue,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct WinCmdCommandVars {
    pub resource: YamlValue,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct PrintCommandVars {
    pub resource: YamlValue,
}

pub type BashCommandTask = PlaybookCommand<String, BashCommandVars>;
pub type WinCmdCommandTask = PlaybookCommand<String, WinCmdCommandVars>;
pub type PrintCommandTask = PlaybookCommand<Option<String>, PrintCommandVars>;


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
        print_banner_blue!("TASK: *** {} *** [St.:{}/Succ.:{}/Fail:{}/Skip:{}/Chg:{}] ***", 
            self.name.as_ref().unwrap_or(&"Unnamed".to_string()),
            self.output.status,
            self.output.success,
            self.output.failed,
            self.output.skipped,
            self.output.changed
        );
        if verbose.len() >= 1 {
            print_info!("Task details: {:?}", self);
            //print_info!("Command: {}", command_str);
        }
        if verbose.len() >= 2 {
            print_banner_yellow!("=== Output Obj ===");
            print_info!("{:?}", self.output);
        }
        else {
            if self.output.stdout != "" {
                print_banner_green!("=== Output ===");
                print_success!("{}", self.output.stdout);
            }
            if self.output.stderr != "" {
                print_banner_red!("=== Errors ===");
                print_error!("{}", self.output.stderr);
            }
        }
    }

    fn output(&self) -> PlaybookCommandOutput {
        self.output.clone()
    }
}

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
        print_banner_blue!("TASK: *** {} *** [St.:{}/Succ.:{}/Fail:{}/Skip:{}/Chg:{}] ***", 
            self.name.as_ref().unwrap_or(&"Unnamed".to_string()),
            self.output.status,
            self.output.success,
            self.output.failed,
            self.output.skipped,
            self.output.changed
        );
        if verbose.len() >= 1 {
            print_info!("Task details: {:?}", self);
            //print_info!("Command: {}", command_str);
        }
        if verbose.len() >= 2 {
            print_banner_yellow!("=== Output Obj ===");
            print_info!("{:?}", self.output);
        }
        else {
            if self.output.stdout != "" {
                print_banner_green!("=== Output ===");
                print_success!("{}", self.output.stdout);
            }
            if self.output.stderr != "" {
                print_banner_red!("=== Errors ===");
                print_error!("{}", self.output.stderr);
            }
        }
    }

    fn output(&self) -> PlaybookCommandOutput {
        self.output.clone()
    }
}

impl PlaybookCommandTrait for PrintCommandTask {
    fn execute(&mut self) {
        self.output = PlaybookCommandOutput::new();
        self.output.set_start_time();

        let command = self.command.clone().unwrap_or("print".to_string());
        let name = self.name.clone().unwrap_or(command.to_string());
        let vars: PrintCommandVars = self.vars.clone();
        let register = self.register.clone().unwrap_or("".to_string());
        let state = self.state.clone().unwrap_or("present".to_string());
        let when = self.when.clone().unwrap_or("true".to_string());


        if when == "false"{
            self.output.message = "Skipped".to_string();
            self.output.skipped = 1;
            self.output.set_end_time();
            return;
        }

        // command as a string that is a tera template inside the resource field
        // need to process that string with the known facts

        //let command_data: String = config_proc::yaml_to_string(&vars.resource).unwrap();
        //let command_input: PrintCommandVars = serde_yaml::from_str(&command_data).unwrap();
        
        let data_str;
        let template = serde_yaml::to_string(&vars.resource).unwrap();

        let processed_temp: String;
        {
            let facts = FACTS.read().unwrap();
            processed_temp = config_proc::process_template(&template, &facts.context).unwrap();
        }

        //println!("processed_temp: {:?}", processed_temp);

        let resource = match serde_yaml::from_str(&processed_temp) {
            Ok(v) => v,
            Err(e) => {
                print_error!("Error: {:?}", e);
                YamlValue::Null
            }
        };

        //println!("resource: {:?}", resource);

        match resource {
            YamlValue::String(_) => {
                // in case it as "{{" and "}}", it is a template
                // in case this is a base64 encoded string, it is a file
                // in case this is a json string, it is a json

                let resource_str = resource.as_str().unwrap();
                if resource_str.contains("{{") && resource_str.contains("}}") {
                    let obj_name:String = config_proc::extract_object_path_from_handlebars(&resource_str);
                    {
                        let facts = FACTS.read().unwrap();
                        let values = facts.context.get(&obj_name).unwrap();
                        //println!("values: {:?}", values);
                        data_str = serde_yaml::to_string(&values).unwrap();
                        self.output.data = serde_yaml::from_str(&data_str).unwrap();
                    }
                }
                else if resource_str.contains("[object]") {
                    let obj_name:String = config_proc::extract_object_path_from_handlebars(&template);
                    //println!("obj_name: {:?}", obj_name);
                    {
                        let facts = FACTS.read().unwrap();
                        let values = facts.context.get(&obj_name).unwrap();
                        //println!("values: {:?}", values);
                        data_str = serde_yaml::to_string(&values).unwrap();
                        self.output.data = serde_yaml::from_str(&data_str).unwrap();
                    }
                }
                else {
                    data_str = resource_str.to_string();
                    self.output.data = serde_yaml::from_str(&data_str).unwrap();
                }

            },
            _ => {
                //println!("values: {:?}", command_input.resource);
                data_str = serde_yaml::to_string(&vars.resource).unwrap();
                self.output.data = serde_yaml::from_str(&data_str).unwrap();
            }
        }

        // Commnad execution zone
        // print_error, print_warning, print_info, print_success
        // print_banner_yellow, print_banner_green, print_banner_red
        println!("name: {}", name);
        if command == "print" {
            println!("{}", data_str);
        } else if command == "debug" {
            print_warning!("{:?}", data_str);
        } else if command == "error" {
            print_error!("{}", data_str);
        } else if command == "warning" {
            print_warning!("{}", data_str);
        } else if command == "info" {
            print_banner_yellow!("{}", data_str);
        } else if command == "success" {
            print_banner_green!("{}", data_str);
        } else {
            println!("{}", data_str);
        }


        self.output.stdout = data_str;

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
            {
                let mut facts = FACTS.write().unwrap();
                facts.context.insert(register, &self.output.data);
            }
        }

        self.output.set_end_time();
    }

    fn display(&self, verbose: Option<String>) {
        let verbose = verbose.unwrap_or("".to_string());
        print_banner_blue!("TASK: *** {} *** [St.:{}/Succ.:{}/Fail:{}/Skip:{}/Chg:{}] ***", 
            self.name.as_ref().unwrap_or(&"Unnamed".to_string()),
            self.output.status,
            self.output.success,
            self.output.failed,
            self.output.skipped,
            self.output.changed
        );
        if verbose.len() >= 1 {
            print_info!("Task details: {:?}", self);
            //print_info!("Command: {}", command_str);
        }
        if verbose.len() >= 2 {
            print_banner_yellow!("=== Output Obj ===");
            print_info!("{:?}", self.output);
        }
        else {
            if self.output.stdout != "" {
                print_banner_green!("=== Output ===");
                print_success!("{}", self.output.stdout);
            }
            if self.output.stderr != "" {
                print_banner_red!("=== Errors ===");
                print_error!("{}", self.output.stderr);
            }
        }
    }

    fn output(&self) -> PlaybookCommandOutput {
        self.output.clone()
    }
}



// register implementation here:
 
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