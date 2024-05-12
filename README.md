![GitHub stars](https://img.shields.io/github/stars/Da4ndo/Hyde-Ext?style=social)
![GitHub forks](https://img.shields.io/github/forks/Da4ndo/Hyde-Ext?style=social)
![GitHub contributors](https://img.shields.io/github/contributors/Da4ndo/Hyde-Ext)
![GitHub last commit](https://img.shields.io/github/last-commit/Da4ndo/Hyde-Ext)

<div align="center">

![hyde_banner](https://raw.githubusercontent.com/Da4ndo/Hyde-Ext/main/assets/hyde-ext_banner.png)

# Hyde-Ext Project

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

## 🌟 Features

<table>
<tr>
<td>

### Installation

Hyde-Ext streamlines the setup by installing:

- Custom configurations for ease of use
- Optional images and scripts
- Key packages from `packages.list` such as:
    - `zoxide`
    - `docker`
    - `ttf-nerd-fonts-symbols`
    - `appimagelauncher`
    - `fd` (fast search tool)
- `swayidle` for idle management
- `ufw` for firewall security

> [!NOTE]
> Every option listed in the installation process is optional and can be customized according to user preferences.

</td>
<td>

![cmd1](https://github.com/Da4ndo/Hyde-Ext/blob/main/assets/cmd1.png)

</td>
</tr>
<tr>
<td>

### Restoration
Post-upgrade, Hyde-Ext facilitates the restoration of custom configurations from backups, allowing users to maintain their personalized settings while staying current with the latest HyDE updates.

</td>
<td>

![cmd2](https://github.com/Da4ndo/Hyde-Ext/blob/main/assets/cmd2.png)

</td>
</tr>
</table>


## 🛠️ Installation

To install Hyde-Ext, utilize the pacman package manager:

```
yay -S hyde-ext
```

## 🔄 Changelog

- **Patch Release [v1.1.0] - 06/15/2024**:
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
