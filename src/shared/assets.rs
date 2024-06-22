pub const ASSETS: &[Asset] = &[
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

