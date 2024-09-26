use crate::shared::common::handle_command_output;
use colored::*;
use std::process::{Command, Stdio};

#[derive(Clone)]
pub struct UfwInstaller {
    pub name: String,
    pub display: String,
    pub description: String,
    pub default: bool,
    pub disabled: bool,
}

impl UfwInstaller {
    pub fn new(
        name: String,
        display: String,
        description: String,
        default: bool,
        disabled: bool,
    ) -> Self {
        UfwInstaller {
            name,
            display,
            description,
            default,
            disabled,
        }
    }

    pub fn install(&self) {
        println!(
            "{} UFW setup tailored for standard users and developers.",
            ":: Installing".blue()
        );

        self.enable_ufw();
        self.set_default_policies();
        self.set_logging();
        self.allow_essential_ports();

        println!("{} applied UFW configuration.", "  -> Successfully".green());
    }

    fn enable_ufw(&self) {
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
            eprintln!(
                "{} Failed to set default deny incoming policy: {}",
                ":: Error:".red(),
                e
            );
            std::process::exit(1);
        }
    }

    fn set_default_policies(&self) {
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
            eprintln!(
                "{} Failed to set default allow outgoing policy: {}",
                ":: Error:".red(),
                e
            );
            std::process::exit(1);
        }
        println!(
            "{}",
            "  -> Default policies set: deny (incoming), allow (outgoing), deny (routed).".blue()
        );
    }

    fn set_logging(&self) {
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
            eprintln!(
                "{} Failed to set logging to medium: {}",
                ":: Error:".red(),
                e
            );
            std::process::exit(1);
        }
        println!("{}", "  -> Logging set to medium.".blue());
    }

    fn allow_essential_ports(&self) {
        println!("{}", ":: Allowing essential ports...".yellow());
        let ports = [
            "80/tcp",
            "443/tcp",
            "3000/tcp",
            "8000/tcp",
            "9090/tcp",
            "24880/tcp",
            "ssh",
        ];
        let descriptions = [
            "Port 80/tcp allowed for HTTP traffic.",
            "Port 443/tcp allowed for HTTPS traffic.",
            "Port 3000/tcp allowed for development server access.",
            "Port 8000/tcp allowed for alternative development server access.",
            "Port 9090/tcp allowed for updog file sharing service.",
            "Port 24880/tcp allowed for custom application traffic.",
            "SSH port allowed for secure shell access.",
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
    }
}
