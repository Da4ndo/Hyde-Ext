use colored::*;
use comfy_table::{Table, Row, Cell, presets::UTF8_FULL, modifiers::UTF8_ROUND_CORNERS};
use inquire::MultiSelect;
use std::process;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::install::INSTALLERS;
use crate::meta::Installer;
use crate::shared::common::get_render_config;
use crate::DEBUG;

// TODO Disable wallpapers, add disable option, and set true in assets.meta

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
    table.load_preset(UTF8_FULL).apply_modifier(UTF8_ROUND_CORNERS);
    table.set_header(Row::from(vec![
        Cell::from("Name").fg(comfy_table::Color::Yellow),
        Cell::from("Description").fg(comfy_table::Color::Yellow)
    ]));

    let (display_texts, default_indices): (Vec<&str>, Vec<usize>) =
        INSTALLERS.iter().enumerate().fold(
            (Vec::new(), Vec::new()),
            |(mut texts, mut indices), (index, installer)| {
                if installer.is_disabled() {
                    table.add_row(Row::from(vec![
                        Cell::from(format!("{} {}", installer.get_name(), "(Not available)")).fg(comfy_table::Color::DarkGrey),
                        Cell::from(installer.get_description()).fg(comfy_table::Color::DarkGrey)
                    ]));

                    return (texts, indices);
                }
                texts.push(installer.get_display());
                if installer.is_default() {
                    indices.push(index);
                }
                table.add_row(Row::from(vec![
                    Cell::from(installer.get_name()),
                    Cell::from(installer.get_description())
                ]));

                (texts, indices)
            },
        );

    println!("{}", table);
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
