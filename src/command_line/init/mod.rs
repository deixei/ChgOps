use crate::{print_error, print_info, print_success, print_warning};
use std::fs;
use std::path::Path;

fn create_folder(path: &Path, description: &str) -> bool {
    if !path.exists() {
        print_info!("Creating {} folder: {:?}", description, path);
        match fs::create_dir_all(path) {
            Ok(_) => {
                print_success!("{} folder created: {:?}", description, path);
                true
            },
            Err(e) => {
                print_error!("Failed to create {} folder: {:?}: {}", description, path, e);
                false
            }
        }
    } else {
        print_warning!("{} folder already exists: {:?}", description, path);
        false
    }
}

fn create_or_update_file(path: &Path, content: &str, description: &str, force_update: bool) {
    if !path.exists() || force_update {
        let action = if path.exists() { "Updating" } else { "Creating" };
        print_info!("{} {} file: {:?}", action, description, path);
        match fs::write(path, content) {
            Ok(_) => {
                print_success!("{} file {}: {:?}", description, action.to_lowercase(), path);
            },
            Err(e) => {
                print_error!("Failed to {} {} file: {:?}: {}", action.to_lowercase(), description, path, e);
            }
        }
    } else {
        print_warning!("{} file already exists: {:?}", description, path);
    }
}

// cargo run -- init --name workspace3 -f
pub fn action_init(playbook_name: &str, template_name: &str, force_update: bool) {
    let playbooks_path = Path::new("./playbooks");
    let playbook_path = Path::new("./playbooks").join(playbook_name);
    let playbook_vars_path = playbook_path.join("vars");
    let playbook_vars_file_path = playbook_vars_path.join("vars.yaml");
    let playbook_files_path = playbook_path.join("playbook.yaml");

    let mut content1 = r#"#!chgops
self: demo-playbook
"#;
    let mut content2 = r#"#!chgops
# Description: This is a simple playbook.
---
name: demo-playbook
"#;
    
    if template_name == "default" {
        content1 = r#"#!chgops
self: 
    name: demo-playbook    
    description: "This is a simple playbook"
"#;
        content2 = r#"#!chgops
# Description: This is a simple playbook.
---
name: playbook
settings:
    name: demo-playbook
    description: "This is a simple playbook"
tasks:
  - dx.core.print:
      command: "info"
      vars:
        resource: "{{ self | as_json }}"
      name: "Information"
      register: info1
"#;
    }

    create_folder(&playbooks_path, "playbooks");
    create_folder(&playbook_path, "playbook");
    create_folder(&playbook_vars_path, "playbook vars");

    create_or_update_file(&playbook_vars_file_path, &content1, "playbook variables", force_update);
    create_or_update_file(&playbook_files_path, &content2, "playbook file", force_update);


}