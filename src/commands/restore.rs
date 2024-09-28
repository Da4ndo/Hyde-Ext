use crate::DEBUG;
use colored::*;
use inquire::ui::{Color, StyleSheet};
use inquire::{Confirm, Select};
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;
use walkdir::WalkDir;

use crate::shared::common::get_render_config;

pub fn start(from: Option<&str>) {
    let debug = DEBUG.load(Ordering::SeqCst);

    let folder_path_result = match from {
        Some("latest") => select_backup_folder(Some("latest")),
        Some(path) => {
            let sanitized_path = Path::new(path).canonicalize().map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid path: {}", e))
            });
            sanitized_path.and_then(|p| {
                if p.exists() && p.is_dir() {
                    Ok(p)
                } else {
                    Err(io::Error::new(io::ErrorKind::NotFound, "Provided path does not exist or is not a directory"))
                }
            })
        }
        None => select_backup_folder(None),
    };

    match folder_path_result {
        Ok(folder_path) => {
            println!("{} {}", "  -> Selected:".yellow(), folder_path.display());
            if let Err(e) = handle_backup_folder(&folder_path, debug) {
                eprintln!("{} {}", "[ERROR]:".red(), e.to_string().red());
            }
        }
        Err(e) => eprintln!("\n{} {}", "[ERROR]:".red(), e.to_string().red()),
    }
}

fn select_backup_folder(latest: Option<&str>) -> io::Result<PathBuf> {
    let home_dir = std::env::var("HOME").unwrap_or_default();
    let backup_root = Path::new(&home_dir).join(".config/cfg_backups");
    let mut folders: Vec<_> = WalkDir::new(backup_root)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
        .collect();

    folders.sort_by_key(|dir| std::cmp::Reverse(dir.file_name().to_owned()));

    if latest == Some("latest") {
        return folders
            .first()
            .map(|entry| entry.path().to_path_buf())
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No backup folders found"));
    }

    let folder_names: Vec<String> = folders
        .iter()
        .map(|dir| dir.file_name().to_string_lossy().into_owned())
        .collect();

    let selection = Select::new("Select a backup folder to restore from >", folder_names)
        .with_page_size(10)
        .with_render_config(
            get_render_config()
                .with_selected_option(Some(StyleSheet::new().with_fg(Color::DarkYellow))),
        )
        .prompt()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let selected_folder = folders
        .iter()
        .find(|entry| entry.file_name().to_string_lossy() == selection)
        .map(|entry| entry.path().to_path_buf())
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Selected folder not found"))?;

    Ok(selected_folder)
}

fn handle_backup_folder(backup_folder: &Path, debug: bool) -> io::Result<()> {
    let home_dir = std::env::var("HOME").unwrap_or_default();
    let config_root = Path::new(&home_dir).join(".config");
    let skip_extensions = ["png", "jpg", "svg"];
    let mut restored_count = 0;

    // Handle config.ctl file
    let config_ctl_path = backup_folder.join(".config/waybar/config.ctl");
    if config_ctl_path.exists() {
        handle_config_ctl(&config_ctl_path, &config_root.join("waybar/config.ctl"), debug)?;
    }

    for entry in WalkDir::new(backup_folder)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file()
                && !skip_extensions.contains(
                    &e.path()
                        .extension()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or(""),
                )
        })
    {
        let relative_path = entry
            .path()
            .strip_prefix(backup_folder)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
            .to_path_buf();
        let target_path = if relative_path.starts_with(".config") {
            config_root.join(relative_path.strip_prefix(".config").unwrap())
        } else {
            config_root.parent().unwrap().join(&relative_path)
        };

        if !target_path.exists() {
            if debug {
                println!(
                    "{} Skipping: {} exists in backup but not in the production configuration.",
                    "  :: Debug:".blue(),
                    target_path.display()
                );
            }
            continue;
        }

        match append_custom_configs(entry.path(), &target_path, debug) {
            Ok(true) => {
                restored_count += 1;
                println!(
                    "{} Restored: {}",
                    "  -> OK:".green(),
                    entry
                        .path()
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                );
            }
            Ok(false) => {}
            Err(e) => {
                eprintln!(
                    "\n{} {}",
                    "   
                    [ERROR]:".red(),
                    e.to_string().red()
                );
            }
        }
    }

    println!(
        "\n{} {}\n",
        "Process completed. Total files restored:".yellow(),
        restored_count
    );

    Ok(())
}

fn handle_config_ctl(backup_path: &Path, current_path: &Path, debug: bool) -> io::Result<()> {
    if debug {
        println!("{} Processing config.ctl", "  :: Debug:".blue());
        println!("{} Backup path: {:?}", "  :: Debug:".blue(), backup_path);
        println!("{} Current path: {:?}", "  :: Debug:".blue(), current_path);
    }

    let backup_content = fs::read_to_string(backup_path)?;
    let current_content = fs::read_to_string(current_path)?;

    let active_line = backup_content.lines()
        .find(|line| line.starts_with('1'))
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No active configuration found in backup config.ctl"))?;

    if debug {
        println!("{} Active line found: {}", "  :: Debug:".blue(), active_line);
    }

    let target_config = &active_line[1..];
    let mut updated_content = String::new();
    let mut found = false;
    let mut already_enabled = false;

    if debug {
        println!("{} Starting line-by-line processing", "  :: Debug:".blue());
    }

    for line in current_content.lines() {
        if &line[1..] == target_config {
            if line.starts_with('1') {
                already_enabled = true;
                if debug {
                    println!("{} Configuration already enabled: {}", "  :: Debug:".blue(), line);
                }
            }
            if debug {
                println!("{} Found exact matching config: {}", "  :: Debug:".blue(), line);
            }
            let updated_line = format!("1{}\n", &line[1..]);
            if debug {
                println!("{} Updated line: {}", "  :: Debug:".blue(), updated_line);
            }
            updated_content.push_str(&updated_line);
            found = true;
        } else {
            let current_line = format!("0{}\n", &line[1..]);
            if debug {
                println!("{} Adding non-matching line: {}", "  :: Debug:".blue(), current_line);
            }
            updated_content.push_str(&current_line);
        }
    }

    if debug {
        println!("{} Final updated_content:\n{}", "  :: Debug:".blue(), updated_content);
    }

    if !found {
        println!("{} Waybar: Exact matching configuration not found in current config.ctl", "  :: Warning:".yellow());
    } else if !already_enabled {
        fs::write(current_path, &updated_content)?;
        println!("{} Updated config.ctl", "  -> OK:".green());
        if debug {
            println!("{} Updated content:\n{}", "  :: Debug:".blue(), updated_content);
        }

        // Run Hyde waybar reload
        println!("{} Running Hyde waybar reload...", "  ->".yellow());
        match std::process::Command::new("Hyde").args(["waybar", "reload"]).output() {
            Ok(output) => {
                if output.status.success() {
                    println!("{} Hyde waybar reload executed successfully", "  -> OK:".green());
                    if debug {
                        println!("{} Hyde waybar reload output:\n{}", "  :: Debug:".blue(), String::from_utf8_lossy(&output.stdout));
                    }
                } else {
                    eprintln!("{} Failed to execute Hyde waybar reload", "  -> Error:".red());
                    if debug {
                        eprintln!("{} Hyde waybar reload error:\n{}", "  :: Debug:".blue(), String::from_utf8_lossy(&output.stderr));
                    }
                }
            },
            Err(e) => eprintln!("{} Failed to execute Hyde waybar reload: {}", "  -> Error:".red(), e),
        }
    } else {
        println!("{} Waybar: Configuration already in use, no changes needed", "  -> INFO:".bright_blue());
    }
    
    Ok(())
}

fn append_custom_configs(
    source_path: &Path,
    target_path: &Path,
    debug: bool,
) -> Result<bool, io::Error> {
    if debug {
        println!(
            "{} Processing {}",
            "  :: Debug:".blue(),
            source_path.display()
        );
    }

    let specific_content = format!(
        "{}\n{}\n{}",
        "# ==============================================================================",
        "# ================== Customized Configurations Below ===========================",
        "# =============================================================================="
    );
    let mut content_to_append = String::new();
    let mut append = false;

    let file = fs::File::open(source_path)?;
    let reader = BufReader::new(file);

    let mut skip_next_line = false;
    let mut pokemon_line_commented = false;

    for line in reader.lines() {
        let line = line?;
        if skip_next_line {
            skip_next_line = false;
            continue;
        }
        if line.trim().starts_with('#') && line.contains("pokemon-colorscripts") {
            pokemon_line_commented = true;
        }
        if line.contains(
            "# ================== Customized Configurations Below ===========================",
        ) {
            append = true;
            skip_next_line = true;
            continue;
        }
        if line.contains("Auto-restored by HyDE-Ext") {
            skip_next_line = true;
            continue;
        }
        if append {
            content_to_append.push_str(&line);
            content_to_append.push('\n');
        }
    }

    if append {
        if debug {
            println!(
                "{} Processing source: {}",
                "  :: Debug:".blue(),
                source_path.display()
            );
            println!(
                "{} Targeting path: {}",
                "  :: Debug:".blue(),
                target_path.display()
            );
        }

        let mut target_file_content = fs::read_to_string(target_path)?;
        if target_file_content.contains(
            "# ================== Customized Configurations Below ===========================",
        ) {
            let proceed = Confirm::new(&format!(
                "{} '{}' already has custom configs. They will be replaced. Continue?",
                "Warning:".yellow(),
                target_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
            ))
            .with_default(false)
            .prompt()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

            if !proceed {
                println!("{} {}", "  -> Skipping:".blue(), target_path.display());
                return Ok(false);
            }

            // Replace existing custom configuration or append to the end
            if let Some(custom_config_start) = target_file_content.find(
                "# ================== Customized Configurations Below ===========================",
            ) {
                target_file_content.truncate(custom_config_start);
            }
        }

        if pokemon_line_commented && target_path.file_name().unwrap_or_default() == ".zshrc" {
            let re = regex::Regex::new(r"^(pokemon-colorscripts.*)$").unwrap();
            target_file_content = re.replace_all(&target_file_content, |caps: &regex::Captures| {
                format!("# {}", &caps[1])
            }).to_string();
            fs::write(target_path, &target_file_content)?;
            println!("{} Pokemon-colorscripts line detected and commented out in .zshrc", "  -> INFO:".bright_blue());
        }

        if !content_to_append.is_empty() {
            if debug {
                println!(
                    "{} Replacing/adding custom configurations in: {}",
                    "  :: Debug:".blue(),
                    target_path.display()
                );
            }

            // Remove the last line of target_file_content
            target_file_content = target_file_content.trim_end().lines().collect::<Vec<&str>>().split_last().map(|(_, rest)| rest.join("\n")).unwrap_or_default();
            target_file_content.push_str("\n\n");
            target_file_content.push_str(&specific_content);
            target_file_content.push('\n');
            target_file_content.push_str(&content_to_append);

            fs::write(target_path, target_file_content)?;

            return Ok(true);
        }
    }

    Ok(false)
}
