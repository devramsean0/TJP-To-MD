use json::{object, JsonValue};
use std::fs;

pub fn process_metadata(json: &JsonValue, path: &String) {
    // Write readme
    fs::write(format!("{}/readme.md", &path), json["readme"].to_string()).unwrap();
    // write changelog
    fs::write(format!("{}/changelog.md", &path), json["changelog"].to_string()).unwrap();
    // Create a metadata file
    let metadata = object!{
        name: json["name"].to_string(),
        version: json["version"].to_string(),
        typeDocJsonParserVersion: json["typeDocJsonParserVersion"].to_string(),
        tJPToMDVersion: std::env!("CARGO_PKG_VERSION"),
    };
    fs::write(format!("{}/metadata.json", &path), metadata.dump()).unwrap();
}