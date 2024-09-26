use crate::shared::common::sanitize_path;
use colored::*;
use reqwest::blocking::get;
use std::io::Write;
use std::{fs, path::Path};

#[derive(Clone)]
pub struct FastFetchInstaller {
    pub name: String,
    pub display: String,
    pub description: String,
    source_url: String,
    target_path: String,
    pub default: bool,
    pub disabled: bool
}

impl FastFetchInstaller {
    pub fn new(
        name: String,
        display: String,
        description: String,
        source_url: String,
        target_path: String,
        default: bool,
        disabled: bool
    ) -> Self {
        let sanitized_path = match sanitize_path(&target_path) {
            Ok(path) => path,
            Err(e) => {
                println!("{} Error sanitizing path: {}", "Error:".red(), e);
                std::process::exit(1);
            }
        };

        FastFetchInstaller {
            name,
            display,
            description,
            source_url,
            target_path: sanitized_path,
            default,
            disabled
        }
    }

    pub fn install(&self) {
        println!(
            "{} FastFetch (alter neofetch) terminal images",
            ":: Installing".blue()
        );

        let urls: Vec<&str> = self.source_url.split('|').collect();

        for url in urls {
            let response = match get(url) {
                Ok(resp) => resp,
                Err(e) => {
                    println!("{} Error downloading source: {}", "Error:".red(), e);
                    std::process::exit(1);
                }
            };

            let file_name = url.split('/').last().unwrap();
            let outpath = Path::new(&self.target_path).join(file_name);

            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }

            let mut outfile = fs::File::create(&outpath).unwrap();
            outfile.write_all(&response.bytes().unwrap()).unwrap();
        }

        println!(
            "{} copied all images to target path.",
            "  -> Successfully ".green()
        );
    }
}
