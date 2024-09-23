use colored::*;
use inquire::Confirm;
use serde::Deserialize;
use std::fs;
use std::sync::Arc;

use crate::install::ConfigInstaller;
use crate::install::FastFetchInstaller;
use crate::install::PackageInstaller;
use crate::install::ScriptInstaller;
use crate::install::UfwInstaller;
use crate::install::WallpaperInstaller;
use crate::install::BunInstaller;
use crate::meta::guard::validate_checksum;
use crate::meta::Installer;
use crate::dynamic_path;

#[derive(Deserialize)]
struct Config {
    name: String,
    display: String,
    description: String,
    source_url: Option<String>,
    default: bool,
    target_path: Option<String>,
    disabled: Option<bool>,
}

#[derive(Deserialize)]
struct AssetsMeta {
    version: String,
    configs: Vec<Config>,
}

pub fn load() -> Vec<Arc<dyn Installer + Send + Sync>> {
    let meta_file = dynamic_path::get("assets.meta");

    let meta_content = fs::read_to_string(&meta_file).unwrap_or_else(|e| {
        eprintln!("{} {}: {}", "[ERROR: Meta File Read]".red(), "Failed to read meta file".red(), e.to_string().red());
        std::process::exit(1);
    });

    let assets_meta: AssetsMeta = serde_json::from_str(&meta_content).unwrap_or_else(|e| {
        eprintln!("{} {}: {}", "[ERROR: Meta File Parse]".red(), "Failed to parse meta file".red(), e.to_string().red());
        std::process::exit(1);
    });

    if !validate_checksum(&meta_file, &assets_meta.version) {
        let proceed = Confirm::new(&format!(
            "{} Checksum validation failed for {}. Do you want to continue?",
            "Warning:".yellow(),
            meta_file.display()
        ))
        .with_default(false)
        .prompt()
        .unwrap_or_else(|e| {
            eprintln!("\n{} {}: {}", "[ERROR: Checksum Confirmation]".red(), "Failed to prompt for confirmation".red(), e.to_string().red());
            std::process::exit(1);
        });
        if !proceed {
            std::process::exit(1);
        }
        println!()
    }

    assets_meta
        .configs
        .into_iter()
        .filter_map(|conf| {
            let source_url = conf.source_url.unwrap_or_default();
            let target_path = conf.target_path.unwrap_or_default();

            match conf.display.as_str() {
                display if display.contains("[CONFIG]") => Some(Arc::new(ConfigInstaller::new(
                    conf.name,
                    conf.display,
                    conf.description,
                    source_url,
                    target_path,
                    conf.default,
                )) as Arc<dyn Installer + Send + Sync>),
                display if display.contains("[IMAGES]") => Some(Arc::new(FastFetchInstaller::new(
                    conf.name,
                    conf.display,
                    conf.description,
                    source_url,
                    target_path,
                    conf.default,
                )) as Arc<dyn Installer + Send + Sync>),
                display if display.contains("[PACKAGE]") => Some(Arc::new(PackageInstaller::new(
                    conf.name,
                    conf.display,
                    conf.description,
                    source_url,
                    conf.default,
                )) as Arc<dyn Installer + Send + Sync>),
                display if display.contains("[SCRIPT]") => Some(Arc::new(ScriptInstaller::new(
                    conf.name,
                    conf.display,
                    conf.description,
                    source_url,
                    target_path,
                    conf.default,
                )) as Arc<dyn Installer + Send + Sync>),
                display if display.contains("[UFW]") => Some(Arc::new(UfwInstaller::new(
                    conf.name,
                    conf.display,
                    conf.description,
                    conf.default,
                )) as Arc<dyn Installer + Send + Sync>),
                display if display.contains("[WALLPAPERS]") => Some(Arc::new(WallpaperInstaller::new(
                    conf.name,
                    conf.display,
                    conf.description,
                    source_url,
                    target_path,
                    conf.default,
                    conf.disabled.unwrap_or(false),
                )) as Arc<dyn Installer + Send + Sync>),
                display if display.contains("BUN") => Some(Arc::new(BunInstaller::new(
                    conf.name,
                    conf.display,
                    conf.description,
                    conf.default,
                )) as Arc<dyn Installer + Send + Sync>),
                _ => {
                    eprintln!("\n{} No handler found for the provided meta:", "[ERROR: Unknown Installer Type]".red());
                    eprintln!("    Name: {}", conf.name.red());
                    eprintln!("    Source URL: {}", source_url.red());
                    eprintln!("    Display: {}", conf.display.red());
                    None
                }
            }
        })
        .collect()
}
