use crate::install::{configs, fastfetch, packages, scripts, ufw};
use crate::shared::assets::{Asset, ASSETS};
use crate::shared::common::get_render_config;
use crate::DEBUG;
use colored::*;
use inquire::MultiSelect;
use std::process;
use std::sync::atomic::Ordering;

use crate::install::Config;

pub fn start() {
    let asset_choices = select_assets();
    for choice in asset_choices {
        log_selected_choice(&choice);
        match choice.name {
            "FastFetchAssets" => fastfetch::install(&choice),
            "BUN" => packages::install(&choice),
            "UFW" => ufw::install(&choice),
            "Hyprland.conf" | "Monitors.conf" | "User-Preferences.conf" | ".zshrc" => {
                configs::install(&choice)
            }
            "Packages" => packages::install(&choice),
            "Layout Automation Script" => scripts::install(&choice),
            "NVM" => packages::install(&choice),
            _ => eprintln!("{} Unknown handler: {}", ":: Error:".red(), choice.name),
        }
    }
}

fn select_assets() -> Vec<Asset> {
    let (display_texts, default_indices): (Vec<&str>, Vec<usize>) = ASSETS.iter().enumerate().fold(
        (Vec::new(), Vec::new()),
        |(mut texts, mut indices), (index, asset)| {
            texts.push(asset.display);
            if asset.default {
                indices.push(index);
            }
            (texts, indices)
        },
    );

    let selections = MultiSelect::new("Select what to install:", display_texts)
        .with_default(&default_indices)
        .with_page_size(10)
        .with_render_config(get_render_config())
        .prompt()
        .unwrap_or_else(|e| {
            eprintln!("{} Failed to select options: {}", ":: Error:".red(), e);
            process::exit(1);
        });

    ASSETS
        .iter()
        .filter(|asset| selections.contains(&asset.display))
        .cloned()
        .collect()
}

fn log_selected_choice(choice: &Asset) {
    if DEBUG.load(Ordering::SeqCst) {
        println!(
            "{} Selected choice details: Name: {}, Display: {}, Description: {}",
            ":: Debug:".blue(),
            choice.name,
            choice.display.bright_yellow(),
            choice.description
        );
    }
}
