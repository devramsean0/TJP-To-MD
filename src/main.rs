mod utils;
mod parsers;

use std::{fs::{self, read_dir}, thread::JoinHandle};
use colored::Colorize;
use clap::{Parser, Subcommand};
use utils::config::Config;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    /// Generates a new config file
    Generate {
        
    }
}

fn main() {
    let cli = Cli::parse();
    println!("{}", "Typedoc Json Parser to Markdown converter Initialized".green());
    match &cli.command {
        Some(Commands::Generate {}) => {
            let config = Config {
                input_dir: "".to_string(),
                output_dir: "".to_string(),
                max_threads: 5,
                regen_all: false
            };
            let toml_string = toml::to_string(&config).unwrap();
            if fs::write("tjp_to_md.toml", toml_string).is_err() {
                println!("{}", "Failed to write config file".red());
            } else {
                println!("{}", "Successfully wrote config file, please go and edit it :)".green());
            
            }
        }
        None => {
            start_parse();
        }
    }
}

fn start_parse() {
    // Parse config file
    let config = utils::config::parse_config();
    // Find a list of packages
    let mut package_directories: Vec<String> = vec![];
    for entry in read_dir(&config.input_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            package_directories.push(path.to_str().unwrap().to_string());
        }
    }
    println!("{} {:?} {} {:?}", "Discovered".green(), package_directories.len(), "package directories:".green(), package_directories);
    // Process each package, in parallel
    let mut handles = vec![];
    for package in package_directories {
        let cloned_config = utils::config::parse_config();
        if (handles.len() as u32) >= cloned_config.max_threads {
            // Wait for a thread to finish
            println!("{}", "Hit max threads, waiting for a thread to finish".yellow());
            let handle: JoinHandle<()> = handles.remove(0);
            handle.join().unwrap();
        }
        let handle = std::thread::spawn(move || {
            println!("{} {:?}", "Processing package:".green(), package);
            // Process package
            parsers::process_package(package); // Use the cloned config variable
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
