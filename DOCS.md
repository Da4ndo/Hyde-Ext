# Hyde-Ext Project Documentation 📄

## Overview
Hyde-Ext is a Rust-based command-line application designed to enhance the HyDE (HyDE_CLI) environment. It automates tasks, installs essential tools, manages configurations, and restores settings from backups.

## Project Structure

### Folders and Files

- **`src/`**: Contains the Rust source files.
  - **[main.rs](https://github.com/Da4ndo/HyDe-Ext/blob/main/src/main.rs)**: Entry point of the application, handling CLI commands and subcommands.
  - **`install/`**: Manages installation of various resources.
    - **[manager.rs](https://github.com/Da4ndo/HyDe-Ext/blob/main/src/install/manager.rs)**: Coordinates the installation process for different asset types.
    - **[configs.rs](https://github.com/Da4ndo/HyDe-Ext/blob/main/src/install/configs.rs)**: Handles the installation of configuration files.
    - **[fastfetch.rs](https://github.com/Da4ndo/HyDe-Ext/blob/main/src/install/fastfetch.rs)**: Manages the installation of FastFetch assets.
    - **[packages.rs](https://github.com/Da4ndo/HyDe-Ext/blob/main/src/install/packages.rs)**: Responsible for installing packages listed in [Cargo.toml](https://github.com/Da4ndo/HyDe-Ext/blob/main/Cargo.toml).
    - **[scripts.rs](https://github.com/Da4ndo/HyDe-Ext/blob/main/src/install/scripts.rs)**: Installs and executes scripts.
  - **[restore.rs](https://github.com/Da4ndo/HyDe-Ext/blob/main/src/restore.rs)**: Handles the restoration of configurations from backups.

- **[assets/](https://github.com/Da4ndo/HyDe-Ext/tree/main/assets/Scripts/Scripts.toml)**: Stores configuration files, scripts, and package lists.
  - **[Configs/](https://github.com/Da4ndo/HyDe-Ext/tree/main/assets/Configs/Configs.toml)**: Configuration files for various applications and environments.
  - **[Scripts/](https://github.com/Da4ndo/HyDe-Ext/tree/main/assets/Scripts/Scripts.toml)**: Shell scripts for automation tasks.
  - **[Packages/](https://github.com/Da4ndo/HyDe-Ext/tree/main/assets/Packages/Packages.toml)**: Lists of packages to be installed.
  - **[FastFetchAssets/](https://github.com/Da4ndo/HyDe-Ext/tree/main/assets/FastFetchAssets/FastFetchAssets.toml)**: Assets for FastFetch customization.

### Installation Functions

- **[install_resources()](https://github.com/Da4ndo/HyDe-Ext/blob/main/src/install/manager.rs)**: Orchestrates the installation of resources based on user selections. It handles different types of assets like configurations, scripts, and packages.
- **[install()](https://github.com/Da4ndo/HyDe-Ext/blob/main/src/main.rs)**: These functions manage the installation of specific types of files. They ensure that target directories exist, copy files, and handle permissions.

### Restoration Function

- **[restore_configs()](https://github.com/Da4ndo/HyDe-Ext/blob/main/src/main.rs)**: Allows users to select a backup folder and restores configurations from it. It handles file conflicts and appends new configurations to existing files.

## Key Features

- **Command Line Interface**: Utilizes [clap](https://github.com/Da4ndo/HyDe-Ext/blob/main/Cargo.toml) for parsing command line arguments and subcommands.
- **Environment Variables**: Uses flags like [DEBUG](https://github.com/Da4ndo/HyDe-Ext/blob/main/src/install/manager.rs) and [FORCE](https://github.com/Da4ndo/HyDe-Ext/blob/main/src/install/configs.rs) to alter the behavior of installations and logging.
- **Error Handling**: Robust error handling with user-friendly messages and safe exits.
- **Logging**: Detailed debug and error logs, especially useful when running in debug mode.