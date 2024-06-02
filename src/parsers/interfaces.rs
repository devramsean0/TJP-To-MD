use std::{fs, path::Path};

use json::JsonValue;
use crate::utils::process_block_tags;

pub fn process_interfaces(json: &JsonValue, path: &String) {
    // Create a classes folder
    let interfaces_path = format!("{}/interfaces", &path);
    if !Path::new(&interfaces_path).exists() {
        if fs::create_dir(&interfaces_path).is_err() {
            println!("Failed to create classes folder: {:?}", &interfaces_path);
            return;
        }
    }
    // For each class, process it if the folder doesn't exist yet
    for interface in json["interfaces"].members() {
        let interface_name = interface["name"].to_string();
        // Actually process the class
        fs::write(format!("{}/{}.md", &interfaces_path, interface_name), process_interface(interface_name, interface)).unwrap();
    }
}

fn process_interface(name: String, json: &JsonValue) -> String {
    let mut segments: Vec<String> = vec![];
    segments.push(format!("# {}", name));
    segments.push(process_block_tags(&json["comment"]["tags"]));
    segments.push(format!("{} \n", json["comment"]["description"]));
    segments.push(format!("[url]({}) on line {}  \n", json["source"]["url"], json["source"]["line"]));

    segments.push("## Properties".to_string());
    for property in json["properties"].members() {
        segments.push(format!("### {}", property["name"]));
        segments.push(process_block_tags(&property["comment"]["blockTags"]));
        segments.push(format!("{} \n", property["comment"]["description"]));
        segments.push(format!("Type: {}  \n", property["type"]["type"]));
        segments.push(format!("[url]({}) on line {}  \n", property["source"]["url"], property["source"]["line"]));
    }
    return segments.join("\n");
}