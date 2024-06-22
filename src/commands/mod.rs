mod install;
mod restore;

use lazy_static::lazy_static;
use std::sync::Mutex;

type InstallFn = Option<fn()>;
type RestoreFn = Option<fn(Option<&str>)>;

lazy_static! {
    static ref INSTALL: Mutex<InstallFn> = Mutex::new(None);
    static ref RESTORE: Mutex<RestoreFn> = Mutex::new(None);
}

pub fn load_install() {
    let mut install = INSTALL.lock().unwrap();
    if install.is_none() {
        *install = Some(install::start);
    }
}

pub fn load_restore() {
    let mut restore = RESTORE.lock().unwrap();
    if restore.is_none() {
        *restore = Some(restore::start);
    }
}

pub fn install() {
    load_install();
    if let Some(install_fn) = *INSTALL.lock().unwrap() {
        install_fn();
    }
}

pub fn restore(from: Option<&str>) {
    load_restore();
    if let Some(restore_fn) = *RESTORE.lock().unwrap() {
        restore_fn(from);
    }
}
