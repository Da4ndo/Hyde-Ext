use colored::*;
use std::process::{Command, Stdio};
use crate::shared::common::handle_command_output;

#[derive(Clone)]
pub struct BunInstaller {
    pub name: String,
    pub display: String,
    pub description: String,
    pub default: bool,
}

impl BunInstaller {
    pub fn new(name: String, display: String, description: String, default: bool) -> Self {
        BunInstaller {
            name,
            display,
            description,
            default,
        }
    }

    pub fn install(&mut self) {
        if self.is_bun_installed() {
            println!(
                "{} Bun is already installed. Do you want to continue with the installation? (y/n)",
                ":: Notice:".yellow()
            );
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            if input.trim().to_lowercase() != "y" {
                println!("{} Installation aborted.", ":: Notice:".yellow());
                return;
            }
        }

        println!(
            "{} Bun setup tailored for standard users and developers.",
            ":: Installing".blue()
        );

        self.run_bun_installer();

        println!("{} applied Bun configuration.", "  -> Successfully".green());
    }

    fn run_bun_installer(&self) {
        println!("{}", ":: Running Bun installer...".yellow());
        let child = Command::new("sh")
            .arg("-c")
            .arg("curl -fsSL https://bun.sh/install | bash")
            .stdout(Stdio::piped())
            .spawn();
        if let Ok(child) = child {
            handle_command_output(child);
        } else if let Err(e) = child {
            eprintln!(
                "{} Failed to run Bun installer: {}",
                ":: Error:".red(),
                e.to_string().red()
            );
            std::process::exit(1);
        }
    }

    fn is_bun_installed(&mut self) -> bool {
        let output = Command::new("sh")
            .arg("-c")
            .arg("bun --version")
            .stdout(Stdio::piped())
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    if crate::DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
                        println!(
                            "{} Bun is already installed. Version: {}",
                            "  :: Debug:".blue(),
                            String::from_utf8_lossy(&output.stdout).trim()
                        );
                    }
                    self.display = format!("{} [Installed]", self.display);
                    true
                } else {
                    if crate::DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
                        println!(
                            "{} Bun is not installed.",
                            "  :: Debug:".blue()
                        );
                    }
                    self.display = format!("{} [Partly Installed]", self.display);
                    false
                }
            }
            Err(e) => {
                if crate::DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
                    println!(
                        "{} Failed to check Bun installation: {}",
                        "  :: Debug:".blue(),
                        e.to_string().red()
                    );
                }
                false
            }
        }
    }
}