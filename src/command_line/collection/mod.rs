/// Initializes a collection within a specified namespace. This involves creating necessary
/// directories and files, and optionally updating existing files if `force_update` is true.
///
/// # Arguments
///
/// * `namespace_name` - The name of the namespace where the collection will be initialized.
/// * `collection_name` - The name of the collection to initialize.
/// * `force_update` - A boolean flag indicating whether to force update existing files.
///
/// # Example
///
/// ```rust
/// collection_init("my_namespace", "my_collection", true);
/// ```
///
/// This will create the following directory structure:
///
/// ```
/// ./collections/my_namespace/my_collection/core
/// ```
///
/// And the following files with their respective contents:
///
/// ```
/// ./collections/my_namespace/vars.yaml
/// ./collections/my_namespace/my_collection/vars.yaml
/// ./collections/my_namespace/my_collection/core/vars.yaml
/// ```
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

pub fn collection_init(namespace_name: &str, collection_name: &str, force_update: bool) {
    if force_update {
        print_info!("Starting Collection Initializing process for: {}.{} in force mode", namespace_name, collection_name);
    } else {
        print_info!("Starting Collection Initializing process for: {}.{}", namespace_name, collection_name);
    }

    //namespace_name must be a non empty string with more than 2 characters
    if namespace_name.len() < 2 {
        print_error!("Invalid namespace name: {}. Namespace name must be a non empty string with more than 2 characters", namespace_name);
        return;
    }

    if collection_name.len() < 2 {
        print_error!("Invalid collection name: {}. Collection name must be a non empty string with more than 2 characters", collection_name);
        return;
    }

    let collections_folder = Path::new("./collections");
    let namespace_folder = collections_folder.join(namespace_name);
    let collection_folder = namespace_folder.join(collection_name);
    let core_folder = collection_folder.join("core");

    create_folder(collections_folder, "collections");
    create_folder(&namespace_folder, "namespace");
    create_folder(&collection_folder, "collection");
    create_folder(&core_folder, "core");

    let namespace_vars_file = namespace_folder.join("vars.yaml");
    let collection_vars_file = collection_folder.join("vars.yaml");
    let core_vars_file = core_folder.join("vars.yaml");

    let namespace_vars = format!("#!chgops.namespace\n{}: level1\n", namespace_name);
    let collection_vars = format!("#!chgops.collection\n{}_{}: level2\n", namespace_name, collection_name);
    let core_vars = format!(r#"#!chgops.core
{}_{}_core: level2
"#, namespace_name, collection_name);

    create_or_update_file(&namespace_vars_file, &namespace_vars, "namespace vars", force_update);
    create_or_update_file(&collection_vars_file, &collection_vars, "collection vars", force_update);
    create_or_update_file(&core_vars_file, &core_vars, "core vars", force_update);

    print_success!("Collection {}.{} initialized.", namespace_name, collection_name);
}

pub fn collection_test(scope: &str) {
    // scope is a combination of namespace and collection names, separated by a dot, e.g. "my_namespace.my_collection"
    let parts: Vec<&str> = scope.split('.').collect();
    if parts.len() != 2 {
        print_error!("Invalid scope format: {}. Expected format: namespace.collection", scope);
        return;
    }
    let namespace_name = parts[0];
    let collection_name = parts[1];
    print_info!("Testing collection: {}", scope);

    //namespace_name must be a non empty string with more than 2 characters
    if namespace_name.len() < 2 {
        print_error!("Invalid namespace name: {}. Namespace name must be a non empty string with more than 2 characters", namespace_name);
        return;
    }

    if collection_name.len() < 2 {
        print_error!("Invalid collection name: {}. Collection name must be a non empty string with more than 2 characters", collection_name);
        return;
    }

    let collections_folder = Path::new("./collections");
    let namespace_folder = collections_folder.join(namespace_name);
    let collection_folder = namespace_folder.join(collection_name);
    let core_folder = collection_folder.join("core");

    if !collections_folder.exists() {
        print_error!("Collections folder not found: {:?}", collections_folder);
    }
    else {
        print_success!("Collections folder found: {:?}", collections_folder);
    }

    if !namespace_folder.exists() {
        print_error!("Namespace folder not found: {:?}", namespace_folder);
    }
    else {
        print_success!("Namespace folder found: {:?}", namespace_folder);
    }

    if !collection_folder.exists() {
        print_error!("Collection folder not found: {:?}", collection_folder);
    }
    else {
        print_success!("Collection folder found: {:?}", collection_folder);
    }

    if !core_folder.exists() {
        print_error!("Core folder not found: {:?}", core_folder);
    }
    else {
        print_success!("Core folder found: {:?}", core_folder);
    }

    let namespace_vars_file = namespace_folder.join("vars.yaml");
    let collection_vars_file = collection_folder.join("vars.yaml");
    let core_vars_file = core_folder.join("vars.yaml");

    if namespace_vars_file.exists() {
        print_success!("Namespace vars file found: {:?}", namespace_vars_file);
    }
    else {
        print_error!("Namespace vars file not found: {:?}", namespace_vars_file);
    }

    if collection_vars_file.exists() {
        print_success!("Collection vars file found: {:?}", collection_vars_file);
    }
    else {
        print_error!("Collection vars file not found: {:?}", collection_vars_file);
    }

    if core_vars_file.exists() {
        print_success!("Core vars file found: {:?}", core_vars_file);
    }
    else {
        print_error!("Core vars file not found: {:?}", core_vars_file);
    }



}
