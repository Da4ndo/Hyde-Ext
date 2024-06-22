use crate::DEBUG;
use colored::*;
use reqwest::blocking::get;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, Stdio};
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct ScriptInstaller {
    pub name: String,
    pub display: String,
    pub description: String,
    source_url: String,
    target_dir: String,
    pub default: bool,
}

impl ScriptInstaller {
    pub fn new(
        name: String,
        display: String,
        description: String,
        source_url: String,
        target_dir: String,
        default: bool,
    ) -> Self {
        ScriptInstaller {
            name,
            display,
            description,
            source_url,
            target_dir,
            default,
        }
    }

    pub fn install(&self) {
        let debug_mode = DEBUG.load(Ordering::SeqCst);

        println!("{} {}", ":: Installing".blue(), self.display);

        // Ensure the target directory exists in the user's home directory
        let home_dir = std::env::var("HOME").unwrap_or_default();
        let target_dir = format!("{}/{}", home_dir, self.target_dir);
        if let Err(e) = fs::create_dir_all(&target_dir) {
            eprintln!(
                "{} Failed to create target directory {}: {}",
                ":: Error:".red(),
                target_dir,
                e
            );
            return;
        }

        // Download the script file from the source URL
        let target_path = format!(
            "{}/{}",
            target_dir,
            self.source_url.split('/').last().unwrap_or_default()
        );

        if debug_mode {
            println!(
                "{} Downloading from {} to {}",
                ":: Debug:".blue(),
                self.source_url,
                target_path
            );
        }

        let script_contents = match get(&self.source_url).and_then(|response| response.text()) {
            Ok(contents) => contents,
            Err(e) => {
                eprintln!(
                    "{} Failed to download script from {}: {}",
                    ":: Error:".red(),
                    self.source_url,
                    e
                );
                return;
            }
        };

        if let Err(e) = File::create(&target_path)
            .and_then(|mut file| file.write_all(script_contents.as_bytes()))
        {
            eprintln!(
                "{} Failed to write script to {}: {}",
                ":: Error:".red(),
                target_path,
                e
            );
            return;
        }

        if debug_mode {
            println!(
                "{} Attempting to execute the script after downloading.",
                ":: Debug:".blue()
            );
        }

        // Execute the script and handle its output asynchronously
        let mut child = Command::new("bash")
            .arg(&target_path)
            .stdout(Stdio::piped()) // Capture standard output
            .spawn()
            .expect("Failed to start script process");

        let stdout = child
            .stdout
            .take()
            .expect("Failed to take stdout of child process");
        let reader = BufReader::new(stdout);

        // Use a separate thread to handle the output
        thread::spawn(move || {
            for line in reader.lines() {
                match line {
                    Ok(line) => println!("    {} {}", "[OUTPUT]".yellow(), line.trim_end()),
                    Err(e) => {
                        eprintln!("    {} Error reading script output: {}", "[ERROR]".red(), e)
                    }
                }
            }
        });

        // Allow the script to run for a limited time before killing it
        thread::sleep(Duration::from_secs(3));
        match child.kill() {
            Ok(_) => {
                if debug_mode {
                    println!("{} Script terminated after 3 seconds.", "::".green());
                }
            }
            Err(e) => eprintln!("{} Failed to terminate script: {}", ":: Error:".red(), e),
        }
        // Attempt to collect the exit status after killing the process
        match child.wait() {
            Ok(status) if status.success() => {
                println!(
                    "{} Script was running successfully before termination.",
                    "  ->".green()
                );
            }
            Ok(status) => match status.code() {
                Some(9) | None if status.signal() == Some(9) => {
                    println!(
                        "{} Script was terminated after 3 seconds with SIGKILL (expected).",
                        "  ->".green()
                    );
                }
                _ => {
                    eprintln!(
                        "{} Script was terminated with unexpected exit code: {}",
                        ":: Error:".red(),
                        status
                    );
                }
            },
            Err(e) => {
                eprintln!(
                    "{} Failed to retrieve script exit status: {}",
                    ":: Error:".red(),
                    e
                );
            }
        }

        println!(
            "{} installed script in {}",
            "  -> Successfully".green(),
            target_path
        );
    }
}
