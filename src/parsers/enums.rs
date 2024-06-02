use std::{fs, path::Path};
use json::JsonValue;

use crate::utils::process_block_tags;

pub fn process_enums(json: &JsonValue, path: &String) {
    let enums_path = format!("{}/enums", &path);
    if !Path::new(&enums_path).exists() {
        if fs::create_dir(&enums_path).is_err() {
            println!("Failed to create classes folder: {:?}", &enums_path);
            return;
        }
    }
    // For each class, process it if the folder doesn't exist yet
    for item in json["enums"].members() {
        let enum_name = item["name"].to_string();
        // Actually process the class
        fs::write(format!("{}/{}.md", &enums_path, enum_name), process_enum(enum_name, item)).unwrap();
    }
}

fn process_enum(name: String, json: &JsonValue) -> String {
    let mut segments: Vec<String> = vec![];
    // Name
    segments.push(format!("# {}", name));
    // External
    if json["external"].as_bool().unwrap() {
        segments.push("This in an external class".to_string());
    }
    // Comments
    segments.push(process_block_tags(&json["comment"]["tags"]));
    segments.push(format!("{} \n", json["comment"]["description"]));
    segments.push(format!("[url]({}) on line {}  \n", json["source"]["url"], json["source"]["line"]));

    // Members
    segments.push("## Members".to_string());
    for member in json["members"].members() {
        segments.push(format!("## {}", member["name"]));
        segments.push(format!("Value: {} \n", member["value"]));
        if !member["comment"]["description"].is_null() {
            segments.push(process_block_tags(&member["comment"]["tags"]));
        }
        segments.push(format!("{} \n", member["comment"]["description"]));
        segments.push(format!("[url]({}) on line {}  \n", member["source"]["url"], member["source"]["line"]));
    }
    return segments.join("\n");
}