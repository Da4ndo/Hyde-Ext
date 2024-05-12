use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn restore_configs() {
    match select_backup_folder() {
        Ok(folder_path) => {
            println!("{} {}", "  -> Selected: ".yellow(), folder_path.display());
            if let Err(e) = process_backup_folder(&folder_path) {
                eprintln!("{} {}", ":: Error:".red(), e);
            }
        }
        Err(e) => eprintln!("{} {}", ":: Error:".red(), e),
    }
}

fn select_backup_folder() -> io::Result<PathBuf> {
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

    let folder_names: Vec<String> = folders
        .iter()
        .map(|dir| dir.file_name().to_string_lossy().into_owned())
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(
            " Choose a backup folder to restore from"
                .yellow()
                .to_string(),
        )
        .default(0)
        .items(&folder_names)
        .interact_opt()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
        .ok_or(io::Error::new(io::ErrorKind::Other, "No selection made"))?;

    Ok(folders[selection].path().to_path_buf())
}

fn process_backup_folder(backup_folder: &Path) -> io::Result<()> {
    let home_dir = std::env::var("HOME").unwrap_or_default();
    let config_root = Path::new(&home_dir).join(".config");
    let skip_extensions = ["png", "jpg", "svg"]; // Define extensions to skip
    let mut count = 0;

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
            if std::env::var("DEBUG").unwrap_or_default() == "true" {
                println!(
                    "{} Skipped: {} exists in backup but not in the production configuration.",
                    ":: Debug:".blue(),
                    target_path.display()
                );
            }
            continue;
        }

        match append_custom_configs(entry.path(), &target_path) {
            Ok(append_success) => {
                if append_success {
                    count += 1;
                    println!(
                        "{} Successfully restored custom configurations for {}",
                        "    ->".green(),
                        entry.path().display()
                    );
                }
            }
            Err(e) => eprintln!(
                "{} Failed to process {}: {}",
                ":: Error:".red(),
                entry.path().display(),
                e
            ),
        }
    }

    println!(
        "\n{} {}\n",
        "Process completed. Total files restored:".yellow(),
        count
    );

    Ok(())
}

fn append_custom_configs(source_path: &Path, target_path: &Path) -> Result<bool, io::Error> {
    if std::env::var("DEBUG").unwrap_or_default() == "true" {
        println!("{} Processing {}", "    ->".blue(), source_path.display());
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
    for line in reader.lines() {
        let line = line?;
        if skip_next_line {
            skip_next_line = false;
            continue;
        }
        if line.contains(
            "# ================== Customized Configurations Below ===========================",
        ) {
            append = true;
            skip_next_line = true; // Skip the very next line as it's part of the specific content
            continue;
        }
        if line.contains("Auto-restored by HyDE-Ext") {
            skip_next_line = true; // Skip the very next line as it's part of the specific content
            continue;
        }
        if append {
            content_to_append.push_str(&line);
            content_to_append.push('\n');
        }
    }

    if append {
        if std::env::var("DEBUG").unwrap_or_default() == "true" {
            println!(
                "{} Processing source: {}",
                "    ->".blue(),
                source_path.display()
            );
            println!(
                "{} Targeting path: {}",
                "    ->".blue(),
                target_path.display()
            );
        }
        println!("{} {}", "    -> Found:".green(), source_path.display());

        let target_file_content = fs::read_to_string(target_path)?;
        if target_file_content.contains(
            "# ================== Customized Configurations Below ===========================",
        ) {
            let proceed = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("{} The file '{}' already contains customized configurations. Do you want to continue restoring?", ":: Warning:".yellow(), target_path.file_name().unwrap_or_default().to_string_lossy()))
                .default(false)
                .interact()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

            if !proceed {
                println!(
                    "{} file: {}",
                    "    -> Skipping".blue(),
                    target_path.display()
                );
                return Ok(false);
            }
        }
    }

    if !content_to_append.is_empty() {
        if std::env::var("DEBUG").unwrap_or_default() == "true" {
            println!(
                "{} Opening file for restored custom configurations: {}",
                "    ::".blue(),
                target_path.display()
            );
        }

        let mut target_file = fs::OpenOptions::new().append(true).open(target_path)?;

        writeln!(target_file)?;
        writeln!(target_file)?;

        writeln!(target_file, "{}", specific_content)?;
        writeln!(
            target_file,
            "#                      Auto-restored by HyDE-Ext"
        )?;
        writeln!(
            target_file,
            "# =============================================================================="
        )?;

        writeln!(target_file, "{}", content_to_append)?;

        return Ok(true);
    }

    Ok(false)
}
