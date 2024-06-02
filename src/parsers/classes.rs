use std::{fs, path::Path};

use json::JsonValue;
use crate::utils::process_block_tags;

pub fn process_classes(json: &JsonValue, path: &String) {
    // Create a classes folder
    let classes_path = format!("{}/classes", &path);
    if !Path::new(&classes_path).exists() {
        if fs::create_dir(&classes_path).is_err() {
            println!("Failed to create classes folder: {:?}", &classes_path);
            return;
        }
    }
    // For each class, process it if the folder doesn't exist yet
    for class in json["classes"].members() {
        let class_name = class["name"].to_string();
        // Actually process the class
        fs::write(format!("{}/{}.md", &classes_path, class_name), process_class(class_name, class)).unwrap();
    }
}

fn process_class(name: String, json: &JsonValue) -> String {
    let mut segments: Vec<String> = vec![];
    // Name
    segments.push(format!("# {}", name));
    // External & Abstract
    if json["external"].as_bool().unwrap() {
        segments.push("This in an external class".to_string());
    }
    if json["abstract"].as_bool().unwrap() {
        segments.push("This class is abstract".to_string());
    }
    // Description
    segments.push(process_block_tags(&json["comment"]["tags"]));
    segments.push(json["comment"]["description"].to_string());
    // Source
    segments.push(format!("## Source\n|Source|Line|\n|-|-|\n|[url]({})|{}|", json["source"]["url"], json["source"]["line"]));
    // Constructor Description
    segments.push(format!("## Constructor"));
    if !json["construct"]["comment"]["description"].is_null() {
        segments.push(process_block_tags(&json["construct"]["comment"]["blockTags"]));
        segments.push(json["construct"]["comment"]["description"].to_string());
    }
    // Constructor Parameters
    if !json["construct"]["parameters"].is_empty() {
        segments.push(format!("### Parameters"));
        segments.push(format!("|Name|Type|Comment|Optional|\n|-|-|-|-|"));
        for param in json["construct"]["parameters"].members() {
            segments.push(format!("|{}|{}|{}|{}|", param["name"], param["type"], param["comment"], param["optional"]));
        }
    }
    // Properties
    if !json["properties"].is_empty() {
        segments.push(format!("## Properties"));
        for prop in json["properties"].members() {
            segments.push(format!("### {}", prop["name"]));
            segments.push(format!("[url]({}) on line {}  \n", prop["source"]["url"], prop["source"]["line"]));
            segments.push(process_block_tags(&prop["comment"]["blockTags"]));
            segments.push(format!("|Type|Accessibility|Abstract|Static|Readonly|Optional|\n|-|-|-|-|-|-|\n|{}|{}|{}|{}|{}|{}|", prop["type"]["name"], prop["accessibility"], prop["abstract"], prop["static"], prop["readonly"], prop["optional"]));   
        }
    }
    // Methods
    if !json["methods"].is_empty() {
        segments.push(format!("## Methods"));
/*         segments.push(format!("|Name|Comment|Static|\n|-|-|-|"));
        for method in json["methods"].members() {
            segments.push(format!("|{}|{}|{}|", method["name"], method["comment"]["description"], method["static"]));
        } */
        for method in json["methods"].members() {
            segments.push(format!("### {}", method["name"]));
            segments.push(format!("[url]({}) on line {}  \n", method["source"]["url"], method["source"]["line"]));
            segments.push(format!("|Accessibility|Abstract|Static|\n|-|-|-|-|\n|{}|{}|{}|", method["accessibility"], method["abstract"], method["static"]));
            segments.push(format!("{}", method["comment"]["description"].to_string()));
            segments.push(process_block_tags(&method["signatures"]["comment"]["blockTags"]));

        }
    }
    return segments.join("\n")
}