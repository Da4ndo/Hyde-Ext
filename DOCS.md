# Hyde-Ext Project Documentation 📄

## Overview
Hyde-Ext is a Rust-based command-line application designed to enhance the HyDE (HyDE_CLI) environment. It automates tasks, installs essential tools, manages configurations, and restores settings from backups.

## Project Structure

### Folders and Files

- **`src/`**: Contains the Rust source files.
  - **`main.rs`**: Entry point of the application, handling CLI commands and subcommands.
  - **`install/`**: Manages installation of various resources.
    - **`manager.rs`**: Coordinates the installation process for different asset types.
    - **`configs.rs`**: Handles the installation of configuration files.
    - **`fastfetch.rs`**: Manages the installation of FastFetch assets.
    - **`packages.rs`**: Responsible for installing packages.
    - **`scripts.rs`**: Installs and executes scripts.
    - **`ufw.rs`**: Manages UFW (Uncomplicated Firewall) configuration.
    - **`wallpapers.rs`**: Handles wallpaper installation and management.
    - **`bun.rs`**: Manages the installation of Bun runtime.
  - **`commands/`**: Contains command implementations.
    - **`install.rs`**: Implements the installation command logic.
    - **`restore.rs`**: Implements the restoration command logic.
  - **`meta/`**: Handles metadata and installer traits.
    - **`loader.rs`**: Loads asset metadata and creates installer instances.
    - **`guard.rs`**: Implements checksum validation for security.
  - **`shared/`**: Contains shared utilities and constants.
    - **`assets.rs`**: Defines asset constants used across the project.
    - **`common.rs`**: Contains common utility functions.

- **`assets/`**: Stores configuration files, scripts, and package lists.
  - **`Configs/`**: Configuration files for various applications and environments.
  - **`Scripts/`**: Shell scripts for automation tasks.
  - **`Packages/`**: Lists of packages to be installed.
  - **`FastFetchAssets/`**: Assets for FastFetch customization.
  - **`Wallpapers/`**: Collection of wallpapers for desktop customization.
  - **`NVM/`**: Node Version Manager configuration.

### Key Components

- **Installer Trait**: Defines a common interface for all installer types, promoting modularity and extensibility.
- **Asset Metadata**: Utilizes a JSON-based metadata file (`assets.meta`) to define installable assets, their sources, and targets.
- **Dynamic Loading**: Implements dynamic loading of installers based on asset metadata, allowing easy addition of new asset types.

## Key Features

- **Command Line Interface**: Utilizes `clap` for parsing command line arguments and subcommands.
- **Environment Variables**: Uses flags like `DEBUG` and `FORCE` to alter the behavior of installations and logging.
- **Error Handling**: Robust error handling with user-friendly messages and safe exits.
- **Logging**: Detailed debug and error logs, especially useful when running in debug mode.
- **Checksum Validation**: Implements checksum validation for downloaded assets to ensure integrity.
- **Lazy Loading**: Utilizes `lazy_static` for efficient loading of installers.
- **Multi-threaded Execution**: Implements multi-threaded execution for script installations, improving performance.
- **UFW Configuration**: Automated setup of UFW with predefined port configurations.
- **Wallpaper Management**: Installs and manages wallpapers, including creating symlinks for themes.
- **Bun Runtime Installation**: Supports installation of the Bun JavaScript runtime.
- **NVM Integration**: Installs and configures Node Version Manager for managing multiple Node.js versions.
- **FastFetch Customization**: Installs custom images for FastFetch, enhancing terminal aesthetics.
- **Backup and Restore**: Implements functionality to backup and restore user configurations.

## Installation Function

Orchestrates the installation of resources based on user selections.
Manage the installation of specific types of files, ensuring target directories exist, copying files, and handling permissions.

## Restoration Function

Allows users to select a backup folder and restores configurations from it, handling file conflicts and appending new configurations to existing files.

## Security Features

- **Checksum Validation**: Implements SHA256 checksum validation for downloaded assets to ensure integrity and prevent tampering.
- **Secure File Handling**: Implements secure file operations to prevent unauthorized access or modifications.

## Customization Options

- **User Preferences**: Supports user-specific configurations through `userprefs.conf`.
- **Shell Customization**: Provides a customized `.zshrc` configuration for enhanced shell experience.
- **Wallpaper Selection**: Offers a wide range of wallpapers for desktop customization.
- **FastFetch Themes**: Allows customization of FastFetch output with various anime-themed images.

## Future Enhancements

- **GUI Integration**: Potential for adding a graphical user interface for easier configuration management.
- **Plugin System**: Consideration for implementing a plugin system to allow third-party extensions.
- **Cloud Sync**: Possibility of integrating cloud synchronization for user configurations.