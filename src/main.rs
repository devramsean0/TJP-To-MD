mod utils;
mod parsers;

use std::{fs::read_dir, thread::JoinHandle};
use colored::Colorize;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
}

fn main() {
    //let cli = Cli::parse();
    println!("{}", "Starting parsing".green());
    start_parse();
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
