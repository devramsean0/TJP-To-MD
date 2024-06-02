use std::{fs, path::Path};
use json::JsonValue;

use crate::utils::process_block_tags;

pub fn process_variables(json: &JsonValue, path: &String) {
    let variables_path = format!("{}/functions", &path);
    if !Path::new(&variables_path).exists() {
        if fs::create_dir(&variables_path).is_err() {
            println!("Failed to create variables folder: {:?}", &variables_path);
            return;
        }
    }
    // For each class, process it if the folder doesn't exist yet
    for item in json["variable"].members() {
        let variable_name = item["name"].to_string();
        // Actually process the class
        fs::write(format!("{}/{}.md", &variables_path, variable_name), process_variable(variable_name, item)).unwrap();
    }
}

fn process_variable(name: String, json: &JsonValue) -> String {
    let mut segments: Vec<String> = vec![];
    // Name
    segments.push(format!("# {}", name));
    segments.push(format!("Type: {}, Value: {}", json["type"]["type"], json["value"]));
    // External
    if json["external"].as_bool().unwrap() {
        segments.push("This in an external variable".to_string());
    }
    // Comments
    segments.push(process_block_tags(&json["comment"]["tags"]));
    segments.push(format!("{} \n", json["comment"]["description"]));
    segments.push(format!("[url]({}) on line {}  \n", json["source"]["url"], json["source"]["line"]));

    return segments.join("\n");
}