mod loader;
mod guard;

use crate::install::{ConfigInstaller, FastFetchInstaller, PackageInstaller, ScriptInstaller, UfwInstaller, BunInstaller, WallpaperInstaller};

pub use loader::load as load;

pub trait Installer {
    fn instal(&self);
    fn get_name(&self) -> &str;
    fn get_display(&self) -> &str;
    fn get_description(&self) -> &str;
    fn is_default(&self) -> bool;
}

macro_rules! impl_installer {
    ($installer:ty) => {
        impl Installer for $installer {
            fn instal(&self) {
                self.install()
            }

            fn get_name(&self) -> &str {
                self.name.as_str()
            }

            fn get_display(&self) -> &str {
                self.display.as_str()
            }

            fn get_description(&self) -> &str {
                self.description.as_str()
            }

            fn is_default(&self) -> bool {
                self.default
            }
        }
    };
}

impl_installer!(ConfigInstaller);
impl_installer!(FastFetchInstaller);
impl_installer!(PackageInstaller);
impl_installer!(ScriptInstaller);
impl_installer!(UfwInstaller);
impl_installer!(BunInstaller);
impl_installer!(WallpaperInstaller);




