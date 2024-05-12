use colored::*;
use std::fs;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;
use std::os::unix::process::ExitStatusExt;

use crate::install::FileEntry;

pub fn install(choice: &FileEntry) {
    let debug_mode = std::env::var("DEBUG").unwrap_or_default() == "true";

    println!(
        "{} {}",
        ":: Installing".blue(),
        choice.title
    );

    // Ensure the target directory exists in the user's home directory
    let home_dir = std::env::var("HOME").unwrap_or_default();
    let target_dir = format!("{}/scripts", home_dir);
    if let Err(e) = fs::create_dir_all(&target_dir) {
        eprintln!("{} Failed to create target directory {}: {}", ":: Error:".red(), target_dir, e);
        return;
    }

    // Copy the script file to the target directory
    if let Some(source_path) = &choice.source_path {
        let target_path = format!("{}/{}", target_dir, source_path.split('/').last().unwrap_or_default());
        if debug_mode {
            println!("{} Copying from {} to {}", ":: Debug:".blue(), source_path, target_path);
        }
        if let Err(e) = fs::copy(source_path, &target_path) {
            eprintln!("{} Failed to copy file from {} to {}: {}", ":: Error:".red(), source_path, target_path, e);
            return;
        }
    } else {
        eprintln!("{} Source path is missing for the script installation.", ":: Error:".red());
        return;
    }

    if debug_mode {
        println!("{} Attempting to execute the script after copying.", ":: Debug:".blue());
    }

    // Construct the command to execute the script
    let target_path = format!("{}/{}", target_dir, choice.source_path.as_ref().unwrap().split('/').last().unwrap_or_default());

    
    // NOTE =============== Warning ===============
    // This command executes scripts like 'auto-layout' which are designed to run in the background
    // to facilitate the initial setup phase.

    // Execute the script and handle its output asynchronously
    let mut child = Command::new("bash")
        .arg(&target_path)
        .stdout(Stdio::piped())  // Capture standard output
        .spawn()
        .expect("Failed to start script process");

    let stdout = child.stdout.take().expect("Failed to take stdout of child process");
    let reader = BufReader::new(stdout);

    // Use a separate thread to handle the output
    thread::spawn(move || {
        for line in reader.lines() {
            match line {
                Ok(line) => println!("    {} {}", "[OUTPUT]".yellow(), line.trim_end()),
                Err(e) => eprintln!("    {} Error reading script output: {}", "[ERROR]".red(), e),
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
        },
        Err(e) => eprintln!("{} Failed to terminate script: {}", ":: Error:".red(), e),
    }
    // Attempt to collect the exit status after killing the process
    match child.wait() {
        Ok(status) if status.success() => {
            println!("{} Script was running successfully before termination.", "  ->".green());
        },
        Ok(status) => {
            match status.code() {
                Some(9) | None if status.signal() == Some(9) => {
                    println!("{} Script was terminated after 3 seconds with SIGKILL (expected).", "  ->".green());
                },
                _ => {
                    eprintln!("{} Script was terminated with unexpected exit code: {}", ":: Error:".red(), status);
                }
            }
        },
        Err(e) => {
            eprintln!("{} Failed to retrieve script exit status: {}", ":: Error:".red(), e);
        }
    }

    println!(
        "{} installed script in {}",
        "  -> Successfully".green(),
        target_path
    );
}
