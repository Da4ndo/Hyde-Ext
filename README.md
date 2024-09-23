![GitHub stars](https://img.shields.io/github/stars/Da4ndo/Hyde-Ext?style=social)
![GitHub forks](https://img.shields.io/github/forks/Da4ndo/Hyde-Ext?style=social)
![GitHub contributors](https://img.shields.io/github/contributors/Da4ndo/Hyde-Ext)
![GitHub last commit](https://img.shields.io/github/last-commit/Da4ndo/Hyde-Ext)
![GitHub license](https://img.shields.io/github/license/Da4ndo/Hyde-Ext)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/Da4ndo/Hyde-Ext)

<div align="center">

![hyde_banner](https://raw.githubusercontent.com/Da4ndo/Hyde-Ext/main/imgs/hyde-ext_banner.png)

# Hyde-Ext: Elevate Your HyDE Experience

Hyde-Ext is a Rust-based command-line application designed to enhance the HyDE (HyDE_CLI) environment. It automates tasks, installs essential tools, manages configurations, and restores settings from backups.

<br>

<a href="#🛠️-installation"><kbd> <br> Installation <br> </kbd></a>&ensp;&ensp;
<a href="#🌟-features"><kbd> <br> Key Features <br> </kbd></a>&ensp;&ensp;
<a href="#🔄-changelog"><kbd> <br> Changelog <br> </kbd></a>&ensp;&ensp;
<a href="https://github.com/Da4ndo/Hyde-Ext/wiki"><kbd> <br> Wiki <br> </kbd></a>&ensp;&ensp;
<a href="https://da4ndo.com"><kbd> <br> Da4ndo Web <br> </kbd></a>&ensp;&ensp;
<a href="https://www.youtube.com/@da4ndo577"><kbd> <br> Da4ndo Youtube <br> </kbd></a>&ensp;&ensp;
</div>
<br><br>

## 🌟 Key Features

<div align="center">

| Installation | Restoration |
|:------------:|:-----------:|
| <img src="https://github.com/Da4ndo/Hyde-Ext/blob/main/imgs/cmd1.png" width="400"> | <img src="https://github.com/Da4ndo/Hyde-Ext/blob/main/imgs/cmd2.png" width="400"> |
| Hyde-Ext streamlines setup with:<br>• Custom configurations<br>• Optional images and scripts<br>• Key packages (`zoxide`, `docker`, etc.)<br>• `swayidle` for idle management<br>• `ufw` for firewall security | Facilitates restoration of custom configs from backups,<br>ensuring personalized settings persist across HyDE updates |

</div>

### 🛠️ Customizable Installation

Hyde-Ext offers a flexible installation process:

- **Configurations**: Tailored for optimal user experience
- **Assets**: Optional images and scripts to enhance your environment
- **Essential Packages**: Curated selection from `packages.list`
  - `zoxide`: Smarter cd command
  - `docker`: Containerization platform
  - `ttf-nerd-fonts-symbols`: Icon-rich font
  - `appimagelauncher`: AppImage management
  - `fd`: Fast and user-friendly search tool
- **System Utilities**:
  - `swayidle`: Efficient idle management
  - `ufw`: Uncomplicated Firewall for enhanced security

> [!TIP]
> Every installation option is customizable to fit your unique preferences and workflow.

### 🔄 Seamless Restoration

After upgrading, Hyde-Ext ensures your environment remains familiar:

- Restores custom configurations from backups
- Preserves your personalized settings
- Keeps you up-to-date with the latest HyDE features

Experience the perfect blend of freshness and familiarity with Hyde-Ext's restoration capabilities.

## 🛠️ Installation

To install Hyde-Ext stable, utilize the the package manager:

```
yay -S hyde-ext
```

## 🔄 Changelog

- **Patch Release [v1.2.0] - [Current Date]**:
  - 🔧 Refactored codebase for improved structure and maintainability
    - Reorganized modules and file structure
    - Implemented new `commands` module for better separation of concerns
  - 🚀 Enhanced installation process
    - Added new `ConfigInstaller` for more flexible configuration handling
    - Improved asset management with new `Asset` struct
  - 🖥️ Updated GitHub Actions workflow
    - Implemented new deployment process for stable versions
    - Added checksums generation for release artifacts
  - 🎨 Updated wallpapers and images
  - 🔒 Removed UFW configuration from default setup
  - 🧰 Added new development tools
    - Integrated `inquire` for improved user prompts
    - Implemented `lazy_static` for better performance
  - 📦 Updated dependencies
    - Added `reqwest` for HTTP requests
  - 🐛 Various bug fixes and performance improvements

- **Patch Release [v1.1.1] - 05/14/2024**:
  - 🛠️ Updated PKGBUILD for AUR package to ensure compatibility and stability.
  - 🚀 Added `hyde-ext-git` package for users preferring the cutting-edge version directly from the git repository.

- **Patch Release [v1.1.0] - 05/13/2024**:
  - 🛠️ Fixed an issue where assets files were not correctly handled in release mode.

- **Initial Release [UNPATCHED] [v1.0.7+beta] - 05/12/2024**:
  - 🚀 First public release of Hyde-Ext.
  - 🛠️ Core functionalities for installation and restoration of HyDE configurations.
  - 🖥️ Basic command-line interface implemented.
  - 🌟 Added new installation options for advanced users.
  - 🔍 Further refinements in the restoration process to handle edge cases.
  - 🐞 General bug fixes and performance improvements.


## 👏 Credits

Hyde-Ext, as an extension project, owes its existence to the configurations created by [prasanthrangan/hyprdots](https://github.com/prasanthrangan/hyprdots/).

## 🤝 Contribution

Contributions are welcome. If you have any suggestions or modifications, feel free to fork this project, make your changes, and submit a pull request.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.
