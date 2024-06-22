use colored::*;
use std::process::Command;

#[derive(Clone)]
pub struct PackageInstaller {
    pub name: String,
    pub display: String,
    pub description: String,
    source_url: String,
    pub default: bool,
}

impl PackageInstaller {
    pub fn new(
        name: String,
        display: String,
        description: String,
        source_url: String,
        default: bool,
    ) -> Self {
        PackageInstaller {
            name,
            display,
            description,
            source_url,
            default,
        }
    }

    pub fn install(&self) {
        let content = match self.download_package_list() {
            Ok(content) => content,
            Err(e) => {
                eprintln!(
                    "{} Failed to download package list file {}: {}",
                    ":: Error:".red(),
                    self.source_url,
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
                        self.install_group(&aur_helper, &current_group);
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
            self.install_group(&aur_helper, &current_group);
        }
    }

    fn download_package_list(&self) -> Result<String, reqwest::Error> {
        let response = reqwest::blocking::get(&self.source_url)?;
        response.text()
    }

    fn install_group(&self, aur_helper: &str, packages: &[&str]) {
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
}
