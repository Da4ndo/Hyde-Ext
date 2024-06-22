use clap::{Command, Arg};
use std::process;
use colored::*;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, Ordering};

mod commands;
mod shared;
mod install;
mod meta;
mod dynamic_path;

lazy_static! {
    pub static ref DEBUG: AtomicBool = AtomicBool::new(false);
    pub static ref FORCE: AtomicBool = AtomicBool::new(false);
}

fn main() {
    let app = Command::new("hyde-ext")
        .arg_required_else_help(true)
        .color(clap::ColorChoice::Always) 
        .version(env!("CARGO_PKG_VERSION"))
        .author("Da4ndo <contact@da4ndo.com>")
        .about("HyDE-Ext is an extension to HyDE (HyDE_CLI) that helps automate tasks, add various custom images/wallpapers, and configurations.")
        .arg(Arg::new("debug")
            .long("debug")
            .global(true) // Make debug flag available everywhere
            .action(clap::ArgAction::SetTrue)
            .help("Sets the debug environment to true"))
        .arg(Arg::new("force")
            .long("force")
            .global(true) // Make force flag available everywhere
            .action(clap::ArgAction::SetTrue)
            .help("Forces the operation to proceed with all warnings and skippings"))
        .subcommand(Command::new("install")
             .about("Installs the specified tool or resource"))
        .subcommand(Command::new("restore")
             .about("Restores the application to its default state")
             .arg(Arg::new("from")
                 .long("from")
                 .value_name("FOLDER")
                 .help("Specifies the backup folder to restore from (e.g., 'latest' or a specific path)")));
        

    let matches = app.clone().try_get_matches().unwrap_or_else(|e| {
        match e.kind() {
            clap::error::ErrorKind::UnknownArgument | clap::error::ErrorKind::InvalidSubcommand => {
                eprintln!("{} {}", ":: Error:".red(), e);
            },
            _ => {
                println!("{}", e);
                // app.print_help().expect("Error printing help information");
            }
        }
        process::exit(1);
    });

    if matches.get_flag("debug") {
        DEBUG.store(true, Ordering::SeqCst);
        println!("{} Debug mode is activated.", "::".green());
    }

    if matches.get_flag("force") {
        FORCE.store(true, Ordering::SeqCst);
        println!("{} Force mode is activated.", "->".green());
    }

    if DEBUG.load(Ordering::SeqCst) {
        if cfg!(debug_assertions) {
            println!("{} Application is running in debug build mode.", ":: Debug:".blue());
        } else {
            println!("{} Application is running in release build mode.", ":: Debug:".blue());
        }
    }

    banner();

    match matches.subcommand() {
        Some(("restore", sub_m)) => {
            let from = sub_m.get_one::<String>("from").map(|s| s.as_str());
            commands::restore(from);
        },
        Some(("install", _)) => {
            commands::install();
        }
        _ => {
            println!("{} For command usage, type --help", ":: Info:".bright_blue());
        }
    }
}

fn banner() {
    let banner_text = r#"
$$\   $$\           $$$$$$$\  $$$$$$$$\   |middle|   $$$$$$$$\             $$\     
$$ |  $$ |          $$  __$$\ $$  _____|  |middle|   $$  _____|            $$ |    
$$ |  $$ |$$\   $$\ $$ |  $$ |$$ |        |middle|   $$ |      $$\   $$\ $$$$$$\   
$$$$$$$$ |$$ |  $$ |$$ |  $$ |$$$$$\ |middle|$$$$$$\ $$$$$\    \$$\ $$  |\_$$  _|  
$$  __$$ |$$ |  $$ |$$ |  $$ |$$  __||middle|\______|$$  __|    \$$$$  /   $$ |    
$$ |  $$ |$$ |  $$ |$$ |  $$ |$$ |      |middle|     $$ |       $$  $$<    $$ |$$\ 
$$ |  $$ |\$$$$$$$ |$$$$$$$  |$$$$$$$$\  |middle|    $$$$$$$$\ $$  /\$$\   \$$$$  |
\__|  \__| \____$$ |\_______/ \________| |middle|    \________|\__/  \__|   \____/ 
          $$\   $$ |                      |middle|                                 
          \$$$$$$  |                      |middle|                                 
           \______/                        |middle|                                
    "#;
    println!();
    for line in banner_text.trim().lines() {
        let parts: Vec<&str> = line.split("|middle|").collect();
        if parts.len() == 2 {
            println!("{}{}", parts[0].purple(), parts[1].bright_green());
        }
    }
    println!();
    let version_info = format!("{}{}{}", "HyDE-Ext Version: ".purple(), "v".green(), env!("CARGO_PKG_VERSION").green());
    let border_length = version_info.len() - 23; // -23 because of colors
    let border = "═".repeat(border_length).purple();
    println!("{}", border);
    println!("{} {} {}", "║".purple(), version_info, "║".purple());
    println!("{}", border);
    println!();
}