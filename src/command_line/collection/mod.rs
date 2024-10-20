use crate::{print_error, print_info, print_success, print_warning};
use std::fs;
use std::path::Path;
pub fn collection_init(namespace_name: &str, collection_name: &str, force_update: bool) {
    print_info!("Initializing collection: {}.{}", namespace_name, collection_name);
    // find a folder with the name "collection" in the current directory
    // if it doesn't exist, create it
    // create a folder with the name "namespace_name" in the "collection" folder if it doesn't exist
    // create a folder with the name "collection_name" in the "namespace_name" folder if it doesn't exist
    // create a folder with the name "core" in the "collection_name" folder if it doesn't exist
    
    let collections_folder = format!("./collections");
    let namespace_folder = format!("{}/{}", collections_folder, namespace_name);
    let collection_folder = format!("{}/{}", namespace_folder, collection_name);
    let core_folder = format!("{}/core", collection_folder);

    let path_collections_folder = Path::new(&collections_folder);
    let path_namespace_folder = Path::new(&namespace_folder);
    let path_collection_folder = Path::new(&collection_folder);
    let path_core_folder = Path::new(&core_folder);

    if !path_collections_folder.exists() {
        print_info!("Creating collections folder: {}", collections_folder);
        let error_msg = format!("Failed to create collections {} folder", collections_folder);

        fs::create_dir_all(&collections_folder).expect(error_msg.as_str());
    } else {
        print_warning!("Collections folder already exists: {}", collections_folder);
    }

    if !path_namespace_folder.exists() {
        print_info!("Creating namespace folder: {}", namespace_folder);
        let error_msg = format!("Failed to create namespace {} folder", namespace_folder);

        fs::create_dir_all(&namespace_folder).expect(error_msg.as_str());
    } else {
        print_warning!("Namespace folder already exists: {}", namespace_folder);
    }

    if !path_collection_folder.exists() {
        print_info!("Creating collection folder: {}", collection_folder);
        let error_msg = format!("Failed to create collection {} folder", collection_folder);

        fs::create_dir_all(&collection_folder).expect(error_msg.as_str());
    } else {
        print_warning!("Collection folder already exists: {}", collection_folder);
    }

    if !path_core_folder.exists() {
        print_info!("Creating core folder: {}", core_folder);
        let error_msg = format!("Failed to create core {} folder", core_folder);

        fs::create_dir_all(&core_folder).expect(error_msg.as_str());
    } else {
        print_warning!("Core folder already exists: {}", core_folder);
    }


    let namespace_vars_file = format!("{}/vars.yaml", namespace_folder);
    let collection_vars_file = format!("{}/vars.yaml", collection_folder);
    let core_vars_file = format!("{}/vars.yaml", core_folder);

    let path_namespace_vars_file = Path::new(&namespace_vars_file);
    let path_collection_vars_file = Path::new(&collection_vars_file);
    let path_core_vars_file = Path::new(&core_vars_file);

    let namespace_vars = format!("#!chgops.namespace\n{}: level1\n", namespace_name);
    let collection_vars = format!("#!chgops.collection\n{}_{}: level2\n", namespace_name, collection_name);
    let core_vars = format!(r#"#!chgops.core
{}_{}_core: level2
"#, namespace_name, collection_name);

    if !path_namespace_vars_file.exists() {
        print_info!("Creating namespace vars file: {}", namespace_vars_file);
        let error_msg = format!("Failed to create namespace {} vars file", namespace_vars_file);

        fs::write(&namespace_vars_file, namespace_vars).expect(error_msg.as_str());
    } else {
        if force_update {
            print_info!("Updating namespace vars file: {}", namespace_vars_file);
            let error_msg = format!("Failed to update namespace {} vars file", namespace_vars_file);

            fs::write(&namespace_vars_file, namespace_vars).expect(error_msg.as_str());
        } else {
            print_warning!("Namespace vars file already exists: {}", namespace_vars_file);
        }
    }

    if !path_collection_vars_file.exists() {
        print_info!("Creating collection vars file: {}", collection_vars_file);
        let error_msg = format!("Failed to create collection {} vars file", collection_vars_file);

        fs::write(&collection_vars_file, collection_vars).expect(error_msg.as_str());
    } else {
        if force_update {
            print_info!("Updating collection vars file: {}", collection_vars_file);
            let error_msg = format!("Failed to update collection {} vars file", collection_vars_file);

            fs::write(&collection_vars_file, collection_vars).expect(error_msg.as_str());
        } else {
            print_warning!("Collection vars file already exists: {}", collection_vars_file);
        }
    }

    if !path_core_vars_file.exists() {
        print_info!("Creating core vars file: {}", core_vars_file);
        let error_msg = format!("Failed to create core {} vars file", core_vars_file);

        fs::write(&core_vars_file, core_vars).expect(error_msg.as_str());
    } else {
        if force_update {
            print_info!("Updating core vars file: {}", core_vars_file);
            let error_msg = format!("Failed to update core {} vars file", core_vars_file);

            fs::write(&core_vars_file, core_vars).expect(error_msg.as_str());
        } else {
            print_warning!("Core vars file already exists: {}", core_vars_file);
        }
    }


}

pub fn collection_test(scope: &str) {
    print_info!("Testing collection: {}", scope);
}