use colored::*;
use std::{fs, path::Path};

use crate::shared::assets::Asset;

pub fn install(_choice: &Asset) {
    println!(
        "{} FastFetch (alter neofetch) terminal images",
        ":: Installing".blue()
    );

    // Define source and target paths locally
    let source_path = "/path/to/source"; // Replace with actual source path
    let target_path = "/path/to/target"; // Replace with actual target path

    match fs::read_dir(source_path) {
        Ok(entries) => {
            let results: Vec<_> = entries
                .filter_map(Result::ok)
                .map(|entry| {
                    let target_file_path = Path::new(target_path).join(entry.file_name());
                    fs::copy(entry.path(), target_file_path)
                })
                .collect();

            if results.iter().all(Result::is_ok) {
                println!(
                    "{} copied all images to target path.",
                    "  -> Successfully ".green()
                );
            } else {
                let errors: Vec<_> = results.into_iter().filter_map(Result::err).collect();
                for error in errors {
                    println!("{} Error copying file: {}", "Error:".red(), error);
                }
                println!(
                    "{} Error copying some assets, see above for details.",
                    "Error:".red()
                );
                std::process::exit(1);
            }
        }
        Err(e) => {
            println!("{} Error reading source directory: {}", "Error:".red(), e);
            std::process::exit(1);
        }
    }
}
