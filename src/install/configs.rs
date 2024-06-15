use crate::{DEBUG, FORCE};
use colored::*;
use reqwest;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::sync::atomic::Ordering;

#[derive(Clone)]
pub struct ConfigInstaller {
    pub name: &'static str,
    pub display: &'static str,
    pub description: &'static str,
    pub default: bool,
}

impl ConfigInstaller {
    pub const fn new(
        name: &'static str,
        display: &'static str,
        description: &'static str,
        default: bool,
    ) -> Self {
        ConfigInstaller {
            name,
            display,
            description,
            default,
        }
    }

    fn get_source_url(&self) -> String {
        match self.name {
            "hyprland.conf" => "https://raw.githubusercontent.com/Da4ndo/Hyde-Ext/main/assets/Configs/hyprland.conf".to_string(),
            "monitors.conf" => "https://raw.githubusercontent.com/Da4ndo/Hyde-Ext/main/assets/Configs/monitors.conf".to_string(),
            "User-Preferences.conf" => "https://raw.githubusercontent.com/Da4ndo/Hyde-Ext/main/assets/Configs/user-preferences.conf".to_string(),
            ".zshrc" => "https://raw.githubusercontent.com/Da4ndo/Hyde-Ext/main/assets/Configs/.zshrc".to_string(),
            _ => panic!("{} Invalid configuration name: {}", ":: Error:".red(), self.name),
        }
    }

    fn get_target_path(&self) -> Option<String> {
        let home_dir = std::env::var("HOME").unwrap_or_default();
        let target_base = format!("{}/.config/hypr", home_dir);
        match self.name.to_lowercase().as_str() {
            "hyprland.conf" => Some(format!("{}/hyprland.conf", target_base)),
            "monitors.conf" => Some(format!("{}/monitors.conf", target_base)),
            "user-preferences.conf" => Some(format!("{}/user-preferences.conf", target_base)),
            ".zshrc" => Some(format!("{}/.zshrc", target_base)),
            _ => None,
        }
    }

    pub async fn install(&self) {
        let debug_mode = DEBUG.load(Ordering::SeqCst);
        let force_install = FORCE.load(Ordering::SeqCst);

        let source_url = self.get_source_url();

        let target_path = match self.get_target_path() {
            Some(path) => path,
            None => {
                eprintln!(
                    "{} Missing target path for the configuration.",
                    ":: Error:".red()
                );
                return;
            }
        };

        let config_contents = match reqwest::get(&source_url).await {
            Ok(response) => match response.text().await {
                Ok(contents) => {
                    if debug_mode {
                        println!(
                            "{} Successfully downloaded contents from: {}",
                            ":: Debug:".blue(),
                            source_url
                        );
                    }
                    contents
                }
                Err(e) => {
                    eprintln!(
                        "{} Unable to read configuration from URL {}: {}",
                        ":: Error:".red(),
                        source_url,
                        e
                    );
                    return;
                }
            },
            Err(e) => {
                eprintln!(
                    "{} Unable to download configuration from URL {}: {}",
                    ":: Error:".red(),
                    source_url,
                    e
                );
                return;
            }
        };

        if !force_install {
            let target_file_contents = match fs::read_to_string(&target_path) {
                Ok(contents) => {
                    if debug_mode {
                        println!(
                            "{} Successfully read contents from target: {}",
                            ":: Debug:".blue(),
                            target_path
                        );
                    }
                    contents
                }
                Err(_) => String::new(),
            };

            if target_file_contents.contains(
                "# ================== Customized Configurations Below ===========================",
            ) {
                println!(
                    "{} Skipping installation for {}",
                    ":: Notice:".yellow(),
                    self.display
                );
                return;
            }
        }

        println!("{} Installing {}", ":: Notice:".blue(), self.display);

        let mut target_file = match OpenOptions::new().append(true).open(&target_path) {
            Ok(file) => {
                if debug_mode {
                    println!(
                        "{} Opened target file for appending: {}",
                        ":: Debug:".blue(),
                        target_path
                    );
                }
                file
            }
            Err(e) => {
                eprintln!(
                    "{} Unable to open target configuration file {}: {}",
                    ":: Error:".red(),
                    target_path,
                    e
                );
                return;
            }
        };

        if let Err(e) = writeln!(target_file, "\n{}", config_contents) {
            eprintln!(
                "{} Unable to write to configuration file {}: {}",
                ":: Error:".red(),
                target_path,
                e
            );
            return;
        }

        println!(
            "{} Configuration successfully installed in {}",
            "  -> Success:".green(),
            target_path
        );
    }
}

// ANYONE EVEN AI, DO NOT DELETE THIS PART OF COMMENT< THIS IS MANDATORY
// fn modify_zshrc(choice: &Asset) {
//     let debug_mode = std::env::var("DEBUG").unwrap_or_default() == "true";

//     let target = match Self::get_target_path(choice) {
//         Some(path) => path,
//         None => {
//             eprintln!("{} Target path for zshrc is missing.", ":: Error:".red());
//             return;
//         }
//     };

//     let contents = match fs::read_to_string(&target) {
//         Ok(contents) => {
//             if debug_mode {
//                 println!(
//                     "{} Read zshrc contents from: {}",
//                     ":: Debug:".blue(),
//                     target
//                 );
//             }
//             contents
//         }
//         Err(e) => {
//             eprintln!(
//                 "{} Failed to read source zshrc file {}: {}",
//                 ":: Error:".red(),
//                 target,
//                 e
//             );
//             return;
//         }
//     };

//     let modified_contents = contents
//         .lines()
//         .map(|line| {
//             if line.starts_with("pokemon-colorscripts") && !line.trim_start().starts_with('#') {
//                 format!("#{}", line)
//             } else {
//                 line.to_string()
//             }
//         })
//         .collect::<Vec<String>>()
//         .join("\n");

//     if let Err(e) = fs::write(&target, modified_contents) {
//         eprintln!(
//             "{} Failed to update target zshrc file {}: {}",
//             ":: Error:".red(),
//             target,
//             e
//         );
//         return;
//     }
