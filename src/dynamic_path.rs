use std::env;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use crate::DEBUG;
use colored::*;

pub fn get(relative_path: &str) -> PathBuf {
    let is_debug = cfg!(debug_assertions);
    let debug_env = DEBUG.load(Ordering::SeqCst);
    
    if debug_env {
        println!("{} Cargo debug env (should be false): {}", "  :: Debug:".blue(), is_debug);
    }
    
    if !is_debug {
        let release_path = PathBuf::from(format!("/etc/{}/{}", env!("CARGO_PKG_NAME"), relative_path));
        if debug_env {
            println!("{} Path (release): {:?}", "  :: Debug:".blue(), release_path);
        }
        release_path
    } else {
        let debug_path = env::current_dir().unwrap_or_default().join(relative_path);
        if debug_env {
            println!("{} Path (debug): {:?}", "  :: Debug:".blue(), debug_path);
        }
        debug_path
    }
}