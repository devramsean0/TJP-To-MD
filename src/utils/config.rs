use serde::Deserialize;
use std::{fs, path::Path};
use toml;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub input_dir: String,
    pub output_dir: String,
    pub max_threads: u32,
    pub regen_all: bool,
}

pub fn parse_config() -> Config {
    let path = Path::new("tjp_to_md.toml");
    let contents = fs::read_to_string(path).unwrap();
    toml::from_str(&contents).unwrap()
}