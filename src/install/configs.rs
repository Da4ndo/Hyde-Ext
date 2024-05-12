use colored::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs;

use crate::install::FileEntry;

pub fn install(choice: &FileEntry) {
    let debug_mode = std::env::var("DEBUG").unwrap_or_default() == "true";

    let source_path = match &choice.source_path {
        Some(path) => path,
        None => {
            eprintln!("{} Source path for config is missing.", ":: Error:".red());
            return;
        }
    };

    let target_path = match &choice.target_path {
        Some(path) => path,
        None => {
            eprintln!("{} Target path for config is missing.", ":: Error:".red());
            return;
        }
    };

    let config_contents = match fs::read_to_string(source_path) {
        Ok(contents) => {
            if debug_mode {
                println!("{} Read contents from source: {}", ":: Debug:".blue(), source_path);
            }
            contents
        },
        Err(e) => {
            eprintln!(
                "{} Failed to read source config file {}: {}",
                ":: Error:".red(),
                source_path,
                e
            );
            return;
        }
    };

    let force_install = std::env::var("FORCE").unwrap_or_default() == "true";
    if !force_install {
        let target_file_contents = match fs::read_to_string(target_path) {
            Ok(contents) => {
                if debug_mode {
                    println!("{} Read contents from target: {}", ":: Debug:".blue(), target_path);
                }
                contents
            },
            Err(_) => String::new(), // If the file doesn't exist or can't be read, treat as empty
        };

        if target_file_contents.contains("# ================== Customized Configurations Below ===========================") {
            println!("{} {}", ":: Skipping".yellow(), choice.title);
            return;
        }
    }

    println!("{} {}", ":: Installing".blue(), choice.title);

    let mut target_file = match OpenOptions::new().append(true).open(target_path) {
        Ok(file) => {
            if debug_mode {
                println!("{} Opened target file for appending: {}", ":: Debug:".blue(), target_path);
            }
            file
        },
        Err(e) => {
            eprintln!(
                "{} Failed to open target config file {}: {}",
                ":: Error:".red(),
                target_path,
                e
            );
            return;
        }
    };

    if let Err(e) = writeln!(target_file, "\n{}", config_contents) {
        eprintln!(
            "{} Failed to write to config file {}: {}",
            ":: Error:".red(),
            target_path,
            e
        );
        return;
    }

    println!(
        "{} installed in {}",
        "  -> Successfully".green(),
        target_path
    );
}

pub fn modify_zshrc(choice: &FileEntry) {
    let debug_mode = std::env::var("DEBUG").unwrap_or_default() == "true";

    let target = match &choice.target_path {
        Some(path) => path,
        None => {
            eprintln!("{} Target path for zshrc is missing.", ":: Error:".red());
            return;
        }
    };

    let contents = match fs::read_to_string(target) {
        Ok(contents) => {
            if debug_mode {
                println!("{} Read zshrc contents from: {}", ":: Debug:".blue(), target);
            }
            contents
        },
        Err(e) => {
            eprintln!(
                "{} Failed to read source zshrc file {}: {}",
                ":: Error:".red(),
                target,
                e
            );
            return;
        }
    };

    let modified_contents = contents
        .lines()
        .map(|line| {
            if line.starts_with("pokemon-colorscripts") && !line.trim_start().starts_with('#') {
                format!("#{}", line)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    if let Err(e) = fs::write(target, modified_contents) {
        eprintln!(
            "{} Failed to update target zshrc file {}: {}",
            ":: Error:".red(),
            target,
            e
        );
        return;
    }

    println!(
        "{} {}",
        "  ->".blue(),
        "pokemon-colorscripts disabled on terminal startup".yellow()
    );
}
