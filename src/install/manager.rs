use colored::*;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use std::{fs, path::Path};
use regex::Regex;

use crate::install::configs;
use crate::install::fastfetch;
use crate::install::packages;
use crate::install::scripts;
use crate::install::ufw;
use crate::install::FileConfig;
use crate::install::FileEntry;

pub fn install_resources() {
    let asset_folders = vec!["Configs", "FastFetchAssets", "Packages", "Scripts", "UFW"];
    let asset_choices = gather_asset_choices(&asset_folders);

    for selected_choice in make_choices(&asset_choices) {
        if std::env::var("DEBUG").unwrap_or_default() == "true" {
            println!(
                "{} Selected choice details: Handler: {}, Title: {}, Source Path: {:?}, Target Path: {:?}",
                ":: Debug:".blue(),
                selected_choice.handler,
                selected_choice.title.bright_yellow(),
                selected_choice.source_path,
                selected_choice.target_path
            );
        }
        
        if selected_choice.handler == "configs" {
            configs::install(&selected_choice);
            if let Some(path) = &selected_choice.target_path {
                if path.contains("zshrc") {
                    configs::modify_zshrc(&selected_choice);
                }
            }
        }
        else if selected_choice.handler == "assets" {
            fastfetch::install(&selected_choice);
        }
        else if selected_choice.handler == "packages" {
            packages::install(&selected_choice);
        }
        else if selected_choice.handler == "scripts" {
            scripts::install(&selected_choice);
        }
        else if selected_choice.handler == "ufw" {
            ufw::install(&selected_choice);
        }
    }
}

fn gather_asset_choices(asset_folders: &[&str]) -> Vec<FileEntry> {
    let mut asset_choices = Vec::new();

    for folder in asset_folders {
        let conf_file_path = if cfg!(release) {
            format!("/usr/share/hyde-ext/assets/{}/{}.toml", folder, folder)
        } else {
            format!("assets/{}/{}.toml", folder, folder)
        };
        if std::env::var("DEBUG").unwrap_or_default() == "true" {
            println!(
                "{} Checking existence for: {}",
                ":: Debug:".blue(),
                conf_file_path.bright_yellow()
            );
        }
        if Path::new(&conf_file_path).exists() {
            if std::env::var("DEBUG").unwrap_or_default() == "true" {
                println!(
                    "{} Found configuration for: {}",
                    "  -> Debug:".blue(),
                    conf_file_path.bright_yellow()
                );
            }
            match fs::read_to_string(&conf_file_path) {
                Ok(contents) => parse_config(folder, &contents, &mut asset_choices),
                Err(e) => log_error_reading_config(folder, &e),
            }
        } else {
            asset_choices.push(FileEntry {
                title: folder.to_string(),
                description: folder.to_string(),
                handler: "".to_string(),
                default: false,
                source_path: None,
                target_path: None,
            });
        }
    }

    if std::env::var("DEBUG").unwrap_or_default() == "true" {
        println!();
    }

    asset_choices
}

fn parse_config(folder: &&str, contents: &str, asset_choices: &mut Vec<FileEntry>) {
    let config: Result<FileConfig, _> = toml::from_str(contents);
    match config {
        Ok(config) => {
            for mut file_entry in config.file {
                let formatted_title = format!("{:50}", file_entry.title); // Ensure title is formatted to occupy 50 characters, padding with spaces if necessary
                let display_text = format!(
                    "{} ● {}",
                    formatted_title, colorize_description(&file_entry.description)
                );
                // Replace '~/' with the actual user's home directory in target_path
                file_entry.target_path = file_entry.target_path.map(|path| {
                    std::env::var("HOME").map(|home_dir| {
                        path.replace('~', &home_dir)
                    }).unwrap_or_else(|_| {
                        eprintln!("{} Unable to retrieve HOME directory.", ":: Error:".red());
                        path
                    })
                });
                asset_choices.push(FileEntry {
                    title: file_entry.title,
                    description: display_text,
                    handler: file_entry.handler,
                    default: file_entry.default,
                    source_path: file_entry.source_path,
                    target_path: file_entry.target_path,
                });
            }
        }
        Err(e) => {
            println!(
                "{} Failed to parse TOML configuration for {}: {}",
                ":: Error:".red(),
                folder,
                e
            );
        }
    }
}

fn log_error_reading_config(folder: &&str, e: &std::io::Error) {
    eprintln!(
        "{} Failed to read configuration file for {}: {}",
        ":: Error:".red(),
        folder,
        e
    );
}

fn make_choices(asset_choices: &[FileEntry]) -> Vec<FileEntry> {
    let mut categorized: Vec<(String, Vec<FileEntry>)> = Vec::new();
    let mut display_texts: Vec<String> = Vec::new();
    let mut defaults: Vec<bool> = Vec::new();

    // Group by handler
    for choice in asset_choices {
        match categorized.iter_mut().find(|(handler, _)| *handler == choice.handler) {
            Some((_, entries)) => entries.push(choice.clone()),
            None => categorized.push((choice.handler.clone(), vec![choice.clone()])),
        }
    }
    
    // Sort groups and add to display_texts with separators
    for (_handler, mut entries) in categorized {
        if let Some(last_entry) = entries.last_mut() {

            let separator = "=".repeat(36).white();
            last_entry.description.push_str(&format!("\n{}\n", separator));
        }
        
        for entry in &mut entries {
            display_texts.push(entry.description.clone());
            defaults.push(entry.default);
        }
        
    }

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(&format!(
            "{}\n",
            "Select what to install:\n [a] All   [space] Toggle   [enter] Confirm".yellow()
        ))
        .items(&display_texts)
        .defaults(&defaults)
        .interact_opt()
        .unwrap_or_else(|e| {
            println!("{} Failed to select options: {}", ":: Error:".red(), e);
            None
        });

    if let Some(indices) = selections {
        indices.iter()
            .filter_map(|&i| {
                if std::env::var("DEBUG").unwrap_or_default() == "true" {
                    println!("{} Processing index: {}, display_text: {}", ":: Debug:".blue(), i, display_texts[i].bright_yellow());
                }
                let found_choice = asset_choices.iter().find(|&choice| {
                    if std::env::var("DEBUG").unwrap_or_default() == "true" {
                        println!("{} Checking if choice title: {} matches display_text: {}", "  -> Debug:".blue(), choice.title.bright_yellow(), display_texts[i].bright_yellow());
                    }
                    display_texts[i].contains(&choice.title)
                });
                if std::env::var("DEBUG").unwrap_or_default() == "true" && found_choice.is_some() {
                    println!("{} Match found for index: {}", "  -> Debug:".blue(), i);
                }
                found_choice
            })
            .cloned()
            .collect()
    } else {
        println!(
            "\n{} Nothing was selected. Aborting installation.",
            ":: Warning:".yellow()
        );
        std::process::exit(1);
    }
}

pub fn colorize_description(description: &str) -> String {
    let re = Regex::new(r"\{color:(\w+)\}(.*?)\{/color\}").unwrap();
    let mut colored_description = description.to_string();

    for cap in re.captures_iter(description) {
        let color = &cap[1];
        let text = &cap[2];
        let colored_text = match color {
            "yellow" => text.yellow().to_string(),
            "blue" => text.blue().to_string(),
            "green" => text.green().to_string(),
            "red" => text.red().to_string(),
            _ => text.to_string(),
        };
        colored_description = colored_description.replacen(&cap[0], &colored_text, 1);
    }

    colored_description

}
