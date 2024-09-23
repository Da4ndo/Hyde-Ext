mod guard;
mod loader;

use crate::install::{
    BunInstaller, ConfigInstaller, FastFetchInstaller, PackageInstaller, ScriptInstaller,
    UfwInstaller, WallpaperInstaller,
};

pub use loader::load;

pub trait Installer {
    fn install(&self);
    fn get_name(&self) -> &str;
    fn get_display(&self) -> &str;
    fn get_description(&self) -> &str;
    fn is_default(&self) -> bool;
    fn is_disabled(&self) -> bool;
}

macro_rules! impl_installer {
    ($installer:ty) => {
        impl Installer for $installer {
            fn install(&self) {
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

            fn is_disabled(&self) -> bool {
                false
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

impl Installer for WallpaperInstaller {
    fn install(&self) {
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

    fn is_disabled(&self) -> bool {
        self.disabled
    }

}