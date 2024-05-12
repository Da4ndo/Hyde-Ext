use colored::*;
use std::process::Command;
use std::process::Stdio;
use std::io::{BufRead, BufReader};

use crate::install::FileEntry;

pub fn install(_choice: &FileEntry) {
    println!(
        "{} UFW setup tailored for standard users and developers.",
        ":: Installing".blue()
    );

    println!("{}", ":: Enabling UFW...".yellow());
    let child = Command::new("sudo")
        .arg("ufw")
        .arg("default")
        .arg("deny")
        .arg("incoming")
        .stdout(Stdio::piped())
        .spawn();
    if let Ok(child) = child {
        handle_command_output(child);
    } else if let Err(e) = child {
        eprintln!("{} Failed to set default deny incoming policy: {}", ":: Error:".red(), e);
        std::process::exit(1);
    }

    let child = Command::new("sudo")
        .arg("ufw")
        .arg("default")
        .arg("allow")
        .arg("outgoing")
        .stdout(Stdio::piped())
        .spawn();
    if let Ok(child) = child {
        handle_command_output(child);
    } else if let Err(e) = child {
        eprintln!("{} Failed to set default allow outgoing policy: {}", ":: Error:".red(), e);
        std::process::exit(1);
    }
    println!("{}", "  -> Default policies set: deny (incoming), allow (outgoing), deny (routed).".blue());

    let child = Command::new("sudo")
        .arg("ufw")
        .arg("logging")
        .arg("on")
        .arg("medium")
        .stdout(Stdio::piped())
        .spawn();
    if let Ok(child) = child {
        handle_command_output(child);
    } else if let Err(e) = child {
        eprintln!("{} Failed to set logging to medium: {}", ":: Error:".red(), e);
        std::process::exit(1);
    }
    println!("{}", "  -> Logging set to medium.".blue());

    println!("{}", ":: Allowing essential ports...".yellow());
    let ports = ["80/tcp", "443/tcp", "3000/tcp", "8000/tcp", "9090/tcp", "24880/tcp", "ssh"];
    let descriptions = [
        "Port 80/tcp allowed for HTTP traffic.",
        "Port 443/tcp allowed for HTTPS traffic.",
        "Port 3000/tcp allowed for development server access.",
        "Port 8000/tcp allowed for alternative development server access.",
        "Port 9090/tcp allowed for updog file sharing service.",
        "Port 24880/tcp allowed for custom application traffic.",
        "SSH port allowed for secure shell access."
    ];

    for (port, description) in ports.iter().zip(descriptions.iter()) {
        let child = Command::new("sudo")
            .arg("ufw")
            .arg("allow")
            .arg(port)
            .stdout(Stdio::piped())
            .spawn();
        if let Ok(child) = child {
            handle_command_output(child);
            println!("{} {}", "  ->".blue(), description.blue());
        } else if let Err(e) = child {
            eprintln!("{} Failed to allow {}: {}", ":: Error:".red(), port, e);
            continue;
        }
    }

    println!(
        "{} applied UFW configuration.",
        "  -> Successfully".green()
    );
    
}

/// Function to handle and print the output of a command.
fn handle_command_output(mut child: std::process::Child) {
    if let Some(output) = child.stdout.take() {
        let reader = BufReader::new(output);
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    if std::env::var("DEBUG").unwrap_or_default() == "true" {
                        println!("[COMMAND OUTPUT]: {}", line)
                    }
                },
                Err(e) => eprintln!("{} Error reading command output: {}", ":: Error:".red(), e),
            }
        }
    }
}