use colored::*;
use sha2::{Sha256, Digest};
use std::fs;
use std::path::Path;

pub fn validate_checksum(file_path: &Path, version: &str) -> bool {
    let file_content = fs::read(file_path).unwrap_or_else(|e| {
        eprintln!("{} {} {}", "[ERROR: CHECKSUM_VALIDATION]:".red(), "Failed to read file:".red(), e.to_string().red());
        std::process::exit(1);
    });

    let mut hasher = Sha256::new();
    hasher.update(&file_content);
    let calculated_hash = hasher.finalize();

    let checksum_file = if !cfg!(debug_assertions) {
        "https://raw.githubusercontent.com/Da4ndo/Hyde-Ext/main/meta.sha256sums"
    } else {
        "./meta.sha256sums"
    };

    let checksum_content = if checksum_file.starts_with("http") {
        reqwest::blocking::get(checksum_file)
            .unwrap_or_else(|e| {
                eprintln!("{} {} {}", "[ERROR: CHECKSUM_FETCH]:".red(), "Failed to fetch checksum file:".red(), e.to_string().red());
                std::process::exit(1);
            })
            .text()
            .unwrap_or_else(|e| {
                eprintln!("{} {} {}", "[ERROR: CHECKSUM_READ]:".red(), "Failed to read response from checksum file:".red(), e.to_string().red());
                std::process::exit(1);
            })
    } else {
        fs::read_to_string(checksum_file).unwrap_or_else(|e| {
            eprintln!("{} {} {}", "[ERROR: CHECKSUM_READ]:".red(), "Failed to read local checksum file:".red(), e.to_string().red());
            std::process::exit(1);
        })
    };

    for line in checksum_content.lines() {
        
        if line.contains(version) {
            let expected_hash = line.split_whitespace().next().unwrap();
            if format!("{:x}", calculated_hash) == expected_hash {
                return true;
            } else {
                println!(
                    "{} {}:\n\t{}: {}\n\t{}:      {:x}\n",
                    "Checksum mismatch for".yellow(),
                    file_path.display(),
                    "expected".green(),
                    expected_hash,
                    "got".green(),
                    calculated_hash
                );
                return false;
            }
        }
    }

    eprintln!("{} {} {} {}", "[ERROR: CHECKSUM_VERSION]:".red(), "Version".red(), version.red(), "not found in checksum file".red());
    std::process::exit(1);
}
