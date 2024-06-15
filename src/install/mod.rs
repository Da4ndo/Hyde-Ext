pub mod configs;
pub mod fastfetch;
pub mod packages;
pub mod scripts;
pub mod ufw;

use configs::ConfigInstaller;

pub const CONFIG_INSTALLERS: &[ConfigInstaller] = &[
    ConfigInstaller::new(
        "Hyprland.conf",
        "Hyprland.conf [CONFIG]",
        "Configuration for Hyprland, required by auto-layout.sh to manage language settings.",
        false,
    ),
    ConfigInstaller::new(
        "Monitors.conf",
        "Monitors.conf [CONFIG]",
        "Configuration for dual-monitor setup: primary 1920x1080, secondary 2560x1080.",
        true,
    ),
    ConfigInstaller::new(
        "User-Preferences.conf",
        "User-Preferences.conf [CONFIG]",
        "User preferences including screenshot key bindings, swaylock settings, and cursor window rules.",
        true,
    ),
    ConfigInstaller::new(
        ".zshrc",
        ".zshrc [CONFIG]",
        "Shell configuration enhancing productivity with zoxide directory jumping, alias for 'ip -c' as default, fastfetch, and cargo environment setup.",
        true,
    ),
];
