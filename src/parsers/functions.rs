use std::{fs, path::Path};
use json::JsonValue;

use crate::utils::process_block_tags;

pub fn process_functions(json: &JsonValue, path: &String) {
    let functions_path = format!("{}/functions", &path);
    if !Path::new(&functions_path).exists() {
        if fs::create_dir(&functions_path).is_err() {
            println!("Failed to create functions folder: {:?}", &functions_path);
            return;
        }
    }
    // For each class, process it if the folder doesn't exist yet
    for item in json["function"].members() {
        let function_name = item["name"].to_string();
        // Actually process the class
        fs::write(format!("{}/{}.md", &functions_path, function_name), process_function(function_name, item)).unwrap();
    }
}

fn process_function(name: String, json: &JsonValue) -> String {
    let mut segments: Vec<String> = vec![];
    // Name
    segments.push(format!("# {}", name));
    // External
    if json["external"].as_bool().unwrap() {
        segments.push("This in an external function".to_string());
    }
    // Comments
    segments.push(process_block_tags(&json["comment"]["tags"]));
    segments.push(format!("{} \n", json["comment"]["description"]));
    segments.push(format!("[url]({}) on line {}  \n", json["source"]["url"], json["source"]["line"]));

    return segments.join("\n");
}