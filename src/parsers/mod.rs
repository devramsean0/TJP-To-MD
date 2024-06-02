mod metadata;
mod classes;
mod enums;
mod interfaces;
mod functions;
mod variable;

use std::{fs, path::Path};
use crate::utils::config::parse_config;
use colored::Colorize;

// All Parser stuff
pub fn process_package(path: String) {
    let config = parse_config();
    let output_path = path.replace(&config.input_dir, &config.output_dir);
    // Create package output path if it doesn't exist
    if !Path::new(&output_path).exists() {
        if fs::create_dir(&output_path).is_err() {
            println!("Failed to create output directory: {:?}", &output_path);
            return;
        }
    }
    // For each version, process it if the folder doesn't exist yet
    for version in fs::read_dir(path).unwrap() {
        if version.as_ref().unwrap().path().is_file() {
            // Fix the version path folder
            let version_folder_path = version.as_ref().unwrap().path().to_string_lossy().replace(&config.input_dir, &config.output_dir).replace(".json", "");
            if Path::new(&version_folder_path).exists() && !config.regen_all {
                println!("{} {:?}", "Skipping version folder:".red(), &version_folder_path);
                continue;
            }
            if fs::create_dir(&version_folder_path).is_err() && !config.regen_all {
                println!("{} {:?}", "Failed to create version folder:".red(), &version_folder_path);
                continue;
            }
            // Actually process the json file
            let file = fs::read_to_string(version.as_ref().unwrap().path()).unwrap();
            let parsed_file = json::parse(file.as_str()).unwrap();
            metadata::process_metadata(&parsed_file, &version_folder_path);
            classes::process_classes(&parsed_file, &version_folder_path);
            enums::process_enums(&parsed_file, &version_folder_path);
            interfaces::process_interfaces(&parsed_file, &version_folder_path);
            functions::process_functions(&parsed_file, &version_folder_path);
            variable::process_variables(&parsed_file, &version_folder_path);
        }
    }
}