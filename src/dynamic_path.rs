use std::env;
use std::path::PathBuf;

pub fn get(relative_path: &str) -> PathBuf {
    let is_release = cfg!(release);
    if is_release {
        PathBuf::from(format!("/usr/share/{}/{}", env!("CARGO_PKG_NAME"), relative_path))
    } else {
        PathBuf::from(relative_path)
    }
}