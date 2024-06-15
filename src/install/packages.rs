use colored::*;
use std::{fs, process::Command};

use crate::shared::assets::Asset;

pub fn install(choice: &Asset) {
    let source_path = match choice.name {
        "Packages" => "path/to/packages/list",
        "BUN" => "path/to/bun/list",
        "NVM" => "path/to/nvm/list",
        _ => {
            eprintln!(
                "{} Unknown package type: {}",
                ":: Error:".red(),
                choice.name
            );
            return;
        }
    };

    let content = match fs::read_to_string(source_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!(
                "{} Failed to read package list file {}: {}",
                ":: Error:".red(),
                source_path,
                e
            );
            return;
        }
    };

    let aur_helper = std::env::var("AURHELPER").unwrap_or_else(|_| "yay".to_string());

    let mut current_group = Vec::new();
    let mut group_title = String::new();
    for line in content.lines() {
        if line.starts_with('#') {
            if line.contains("=====") {
                if !current_group.is_empty() {
                    println!("\n{}", group_title);
                    install_group(&aur_helper, &current_group);
                    current_group.clear();
                }
                group_title = line.trim_matches('#').trim().to_string();
            }
        } else if !line.trim().is_empty() {
            current_group.push(line.trim());
        }
    }

    if !current_group.is_empty() {
        println!("\n{}", group_title);
        install_group(&aur_helper, &current_group);
    }
}

fn install_group(aur_helper: &str, packages: &[&str]) {
    println!(
        "{} Installing package group: {:?}",
        ":: Installing".blue(),
        packages
    );
    let status = Command::new(aur_helper)
        .arg("--answerclean")
        .arg("None")
        .arg("--answerdiff")
        .arg("None")
        .arg("-S")
        .arg("--needed")
        .args(packages)
        .status();

    match status {
        Ok(status) if status.success() => {
            println!(
                "{} installed packages: {:?}",
                "  -> Successfully".green(),
                packages
            );
        }
        Ok(_) | Err(_) => {
            eprintln!(
                "{} Failed to install packages: {:?}",
                "Error:".red(),
                packages
            );
            std::process::exit(1);
        }
    }
}
