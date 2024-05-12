use serde::Deserialize;

mod configs;
mod fastfetch;
mod packages;
mod scripts;
mod ufw;
pub mod manager;

#[derive(Deserialize, Debug)]
struct FileConfig {
    file: Vec<FileEntry>,
}

#[derive(Deserialize, Debug, Clone)]
struct FileEntry {
    title: String,
    description: String,
    handler: String,
    #[serde(default)]
    default: bool,
    #[serde(default)]
    source_path: Option<String>,
    #[serde(default)]
    target_path: Option<String>,
}