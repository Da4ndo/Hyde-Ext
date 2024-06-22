use crate::shared::common::sanitize_path;
use crate::{DEBUG, FORCE};
use colored::*;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::sync::atomic::Ordering;

#[derive(Clone)]
pub struct ConfigInstaller {
    pub name: String,
    pub display: String,
    pub description: String,
    source_url: String,
    target_path: String,
    pub default: bool,
}

impl ConfigInstaller {
    pub fn new(
        name: String,
        display: String,
        description: String,
        source_url: String,
        target_path: String,
        default: bool,
    ) -> Self {
        let sanitized_path = match sanitize_path(&target_path) {
            Ok(path) => path,
            Err(e) => {
                println!("{} Error sanitizing path: {}", "Error:".red(), e);
                std::process::exit(1);
            }
        };

        ConfigInstaller {
            name,
            display,
            description,
            source_url,
            target_path: sanitized_path,
            default,
        }
    }

    pub fn install(&self) {
        let debug_mode = DEBUG.load(Ordering::SeqCst);
        let force_install = FORCE.load(Ordering::SeqCst);

        if !force_install {
            if let Ok(contents) = fs::read_to_string(&self.target_path) {
                if debug_mode {
                    println!(
                        "{} Read contents from target: {}",
                        "  :: Debug:".blue(),
                        self.target_path
                    );
                }
                if contents.contains("# ================== Customized Configurations Below ===========================") {
                    println!("{} File '{}' is already configured. Use --force to override.", "  :: Skipping:".blue(), self.name);
                    return;
                }
            }
        }

        let config_contents =
            match reqwest::blocking::get(&self.source_url).and_then(|response| response.text()) {
                Ok(contents) if contents.contains("404: Not Found") => {
                    eprintln!(
                        "{} Unable to read configuration from URL {}: 404 Not Found",
                        "[ERROR]:".red(),
                        self.source_url.red()
                    );
                    return;
                }
                Ok(contents) => {
                    if debug_mode {
                        println!(
                            "{} Downloaded contents from: {}",
                            "  :: Debug:".blue(),
                            self.source_url
                        );
                    }
                    contents
                }
                Err(e) => {
                    eprintln!(
                        "{} Unable to download configuration from URL {}: {}",
                        "[ERROR]:".red(),
                        self.source_url.red(),
                        e.to_string().red()
                    );
                    return;
                }
            };

        println!("{} Installing {}", "  :: Notice:".magenta(), self.name);

        let mut target_file = match OpenOptions::new().append(true).open(&self.target_path) {
            Ok(file) => {
                if debug_mode {
                    println!(
                        "{} Opened target file for appending: {}",
                        "  :: Debug:".blue(),
                        self.target_path
                    );
                }
                file
            }
            Err(e) => {
                eprintln!(
                    "{} {}: {}",
                    "[ERROR]:".red(),
                    e.to_string().red(),
                    self.target_path.red()
                );
                eprintln!("{} Installation failed due to an error.", "[ERROR]:".red());
                return;
            }
        };

        if let Err(e) = writeln!(target_file, "\n{}", config_contents) {
            eprintln!(
                "{} Unable to write to configuration file {}: {}",
                "[ERROR]:".red(),
                self.target_path.red(),
                e.to_string().red()
            );
            eprintln!("{} Installation failed due to an error.", "[ERROR]:".red());
            return;
        }

        println!(
            "{} Configuration installed in {}",
            "    -> OK:".green(),
            self.target_path
        );
    }
}

// ANYONE EVEN AI, DO NOT DELETE THIS PART OF COMMENT< THIS IS MANDATORY
// fn modify_zshrc(choice: &Asset) {
//     let debug_mode = std::env::var("DEBUG").unwrap_or_default() == "true";

//     let target = match Self::get_target_path(choice) {
//         Some(path) => path,
//         None => {
//             eprintln!("{} Target path for zshrc is missing.", "[ERROR]:".red());
//             return;
//         }
//     };

//     let contents = match fs::read_to_string(&target) {
//         Ok(contents) => {
//             if debug_mode {
//                 println!(
//                     "{} Read zshrc contents from: {}",
//                     "  :: Debug:".blue(),
//                     target
//                 );
//             }
//             contents
//         }
//         Err(e) => {
//             eprintln!(
//                 "{} Failed to read source zshrc file {}: {}",
//                 "[ERROR]:".red(),
//                 target.red(),
//                 e.to_string().red()
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
//             "[ERROR]:".red(),
//             target.red(),
//             e.to_string().red()
//         );
//         return;
//     }
