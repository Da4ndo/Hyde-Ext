use colored::*;
use inquire::MultiSelect;
use prettytable::{row, Table};
use std::process;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::install::INSTALLERS;
use crate::meta::Installer;
use crate::shared::common::get_render_config;
use crate::DEBUG;

pub fn start() {
    let selected_installers = select_installers();
    for choice in selected_installers {
        if DEBUG.load(Ordering::SeqCst) {
            println!(
                "{} Selected choice details: Name: {}, Display: {}, Description: {}",
                ":: Debug:".blue(),
                choice.get_name(),
                choice.get_display().bright_yellow(),
                choice.get_description()
            );
        }
        choice.install();
    }
}

fn select_installers() -> Vec<Arc<dyn Installer + Send + Sync>> {
    let mut table = Table::new();
    table.add_row(row!["Name", "Description"]);

    let (display_texts, default_indices): (Vec<&str>, Vec<usize>) =
        INSTALLERS.iter().enumerate().fold(
            (Vec::new(), Vec::new()),
            |(mut texts, mut indices), (index, installer)| {
                texts.push(installer.get_display());
                if installer.is_default() {
                    indices.push(index);
                }
                table.add_row(row![installer.get_name(), installer.get_description()]);
                (texts, indices)
            },
        );

    table.printstd();
    println!();

    let selections = MultiSelect::new("Select what to install:", display_texts)
        .with_default(&default_indices)
        .with_page_size(10)
        .with_render_config(get_render_config())
        .prompt()
        .unwrap_or_else(|e| {
            eprintln!("\n{} {}", "[ERROR]:".red(), e.to_string().red());
            process::exit(1);
        });

    INSTALLERS
        .iter()
        .filter(|installer| selections.contains(&installer.get_display()))
        .cloned()
        .collect()
}
