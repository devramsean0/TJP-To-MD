use json::JsonValue;

pub mod config;

pub fn process_block_tags(tags: &JsonValue) -> String {
    let mut segments: Vec<String> = vec![];
    for tag in tags.members() {
        segments.push(format!("@{}: {} \n", tag["name"], tag["text"]));
    }
    segments.join("\n")
}