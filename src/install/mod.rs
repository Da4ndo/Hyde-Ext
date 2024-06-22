mod bun;
mod configs;
mod fastfetch;
mod packages;
mod scripts;
mod ufw;
mod wallpapers;

use crate::meta::Installer;
use std::sync::Arc;

pub use bun::BunInstaller;
pub use configs::ConfigInstaller;
pub use fastfetch::FastFetchInstaller;
pub use packages::PackageInstaller;
pub use scripts::ScriptInstaller;
pub use ufw::UfwInstaller;
pub use wallpapers::WallpaperInstaller;

lazy_static::lazy_static! {
    pub static ref INSTALLERS: Vec<Arc<dyn Installer + Send + Sync>> = crate::meta::load();
}
