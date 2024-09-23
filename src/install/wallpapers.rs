use crate::shared::common::sanitize_path;
use colored::*;
use reqwest::blocking::get;
use std::io::Write;
use std::{fs, path::Path};

#[derive(Clone)]
pub struct WallpaperInstaller {
    pub name: String,
    pub display: String,
    pub description: String,
    source_url: String,
    target_path: String,
    pub default: bool,
    pub disabled: bool,
}

impl WallpaperInstaller {
    pub fn new(
        name: String,
        display: String,
        description: String,
        source_url: String,
        target_path: String,
        default: bool,
        disabled: bool,
    ) -> Self {
        let sanitized_path = match sanitize_path(&target_path) {
            Ok(path) => path,
            Err(e) => {
                println!("{} Error sanitizing path: {}", "Error:".red(), e);
                std::process::exit(1);
            }
        };

        WallpaperInstaller {
            name,
            display,
            description,
            source_url,
            target_path: sanitized_path,
            default,
            disabled,
        }
    }

    pub fn install(&self) {
        println!("{} Wallpapers for Hyde themes", ":: Installing".blue());

        let urls: Vec<&str> = self.source_url.split('|').collect();
        let target_path = &self.target_path;

        for url in urls {
            let response = match get(url) {
                Ok(resp) => resp,
                Err(e) => {
                    println!("{} Error downloading source: {}", "Error:".red(), e);
                    std::process::exit(1);
                }
            };

            let file_name = url.split('/').last().unwrap();
            let outpath = Path::new(target_path).join(file_name);

            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }

            let mut outfile = fs::File::create(&outpath).unwrap();
            outfile.write_all(&response.bytes().unwrap()).unwrap();
        }

        self.create_symlinks(target_path);

        println!(
            "{} copied all wallpapers to target path.",
            "  -> Successfully ".green()
        );
    }

    // TODO fix wallpapers path
    fn create_symlinks(&self, target_path: &str) {
        let home_dir = std::env::var("HOME").unwrap();
        let themes_path = Path::new(&home_dir).join(".config/hyde/themes");

        if themes_path.exists() {
            for entry in fs::read_dir(themes_path).unwrap() {
                let entry = entry.unwrap();
                let theme_path = entry.path();
                if theme_path.is_dir() {
                    let symlink_path = theme_path.join("wallpapers");
                    if symlink_path.exists() {
                        fs::remove_file(&symlink_path).unwrap();
                    }
                    std::os::unix::fs::symlink(target_path, symlink_path).unwrap();
                }
            }
        } else {
            println!("{} Themes directory does not exist.", "Warning:".yellow());
        }
    }
}
