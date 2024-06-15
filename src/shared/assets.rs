#[derive(Clone)]
pub struct Asset {
    pub name: &'static str,
    pub display: &'static str,
    pub description: &'static str,
    pub default: bool,
}

pub const ASSETS: &[Asset] = &[
    // CONFIGS
    Asset {
        name: "Hyprland.conf",
        display: "Hyprland.conf [CONFIG]",
        description: "Configuration for Hyprland, required by auto-layout.sh to manage language settings.",
        default: false,
    },
    Asset {
        name: "Monitors.conf",
        display: "Monitors.conf [CONFIG]",
        description: "Configuration for dual-monitor setup: primary {color:blue}1920x1080{/color}, secondary {color:blue}2560x1080{/color}.",
        default: true,
    },
    Asset {
        name: "User-Preferences.conf",
        display: "User-Preferences.conf [CONFIG]",
        description: "User preferences including screenshot key bindings, swaylock settings, and cursor window rules.",
        default: true,
    },
    Asset {
        name: ".zshrc",
        display: ".zshrc [CONFIG]",
        description: "Shell configuration enhancing productivity with {color:blue}zoxide{/color} directory jumping, alias for {color:blue}'ip -c'{/color} as default, {color:blue}fastfetch{/color}, and {color:blue}cargo{/color} environment setup.",
        default: true,
    },
    
    // SCRIPTS
    Asset {
        name: "Layout Automation Script",
        display: "Layout Automation Script [SCRIPTS]",
        description: "Automatically adjusts keyboard layout based on settings. {color:yellow}[REQUIRES]: Hyprland.conf [CONFIG]{/color}",
        default: false,
    },
    
    Asset {
        name: "Packages",
        display: "Packages [PACKAGES]",
        description: "Installs some utilities, tools and more.",
        default: true,
    },
    
    Asset {
        name: "BUN",
        display: "BUN Setup [BUN]",
        description: "This configuration installs Bun, a fast all-in-one JavaScript runtime.",
        default: false,
    },
    Asset {
        name: "NVM",
        display: "NVM Configuration [NVM]",
        description: "NVM setup for managing multiple Node.js versions.",
        default: false,
    },
    Asset {
        name: "FastFetchAssets",
        display: "Fastfetch Images [IMAGES]",
        description: "Adding images to {color:blue}fastfetch (alter of neofetch){/color} terminal",
        default: true,
    },
    Asset {
        name: "UFW",
        display: "UFW Configuration [UFW]",
        description: "UFW setup tailored for standard users and developers. Permitted ports include: {color:blue}22, 80, 443, 3000, 8000, 9090, 24880{/color}. {color:yellow}[REQUIRES]: Sudo privileges.{/color}",
        default: true,
    },
];

