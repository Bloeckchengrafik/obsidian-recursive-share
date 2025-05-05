# 🔄 Obsidian Recursive Share

*Version: 1.0.0 | Requires Obsidian: 0.15.0+*

## 📋 Table of Contents

- [Overview](#-overview)
- [Features](#-features)
- [Installation](#-installation)
    - [From Obsidian Community Plugins](#from-obsidian-community-plugins)
    - [Manual Installation](#manual-installation)
    - [Setting up the backend](#setting-up-the-backend)
- [Usage](#-usage)
- [Configuration](#-configuration)
- [Contributing](#-contributing)
- [License](#-license)
- [Support](#-support)

## 🌟 Overview

With Obsidian Recursive Share, you can create share links including images, drawings, and other embeds.

Obsidian Recursive Share is a **third-party plugin** for [Obsidian](https://obsidian.md). 

## ✨ Features

- 📤 Share individual notes with others
- 🔄 Include linked attachments
- 🔒 Control who can share by configuring the backend
- 🌐 Generate shareable links for your content
- 🖼️ Support for embedded images and other media

## 📥 Installation

### From Obsidian Community Plugins

1. Open Obsidian
2. Go to Settings → Community plugins
3. Disable Safe mode if it's enabled
4. Click on Browse and search for "Recursive Share"
5. Click Install, then Enable

### Manual Installation

1. Download the latest release from
   the [GitHub releases page](https://github.com/Bloeckchengrafik/obsidian-recursive-share/releases)
2. Extract the ZIP file into your Obsidian vault's `.obsidian/plugins/` folder
3. Enable the plugin in Obsidian's Community Plugins settings

### Setting up the backend

To set up the backend, use the docker image provided
at [cbergschneider/recursive-share-backend](https://hub.docker.com/r/cbergschneider/recursive-share-backend).
Provide a config file as a mount at `/app/settings.toml`. This could look like this:

```toml
port = 8181
auth_tokens = [ "development" ]
storage_path = "storage"
```

With this configuration, you should add a volume to `/app/storage` and a port forward for port `8181`.

To configure rocket further, you can add a `rocket.toml` to `/app`.
See [their docs](https://rocket.rs/guide/v0.4/configuration/) for more info.

## 🚀 Usage

1. Set up the backend service and connect it to your vault
2. Use Ctrl+P to open the command palette
3. Execute the "Recursive Share: Share Recursively"
4. ???
5. Profit

## ⚙️ Configuration

The plugin requires the following settings:

- **Server URI**: The URI of the sharing server
- **API Key**: Your personal API key for authentication

You can configure these in the plugin settings tab.

## 👥 Contributing

We welcome contributions to Obsidian Recursive Share! Whether it's bug fixes, feature additions, or documentation
improvements, your help is appreciated.

### 🛣️ Contribution Workflow

1. **Fork the Repository**: Start by forking the repository on GitHub.

2. **Create a Branch**: Create a branch for your work.
   ```bash
   git checkout -b feature/your-amazing-feature
   ```

3. **Make Your Changes**: Implement your changes, following the existing code style.

4. **Test Thoroughly**: Ensure your changes work as expected and don't break existing functionality.

5. **Update Documentation**: Update the README or other documentation if necessary.

6. **Submit a Pull Request**: Push your changes to your fork and submit a pull request.
   ```bash
   git push origin feature/your-amazing-feature
   ```

### 📋 Contribution Guidelines

- **Code Style**: Follow the existing code style and TypeScript best practices.
- **Commit Messages**: Write clear, concise commit messages that explain your changes.
- **Documentation**: Add comments to your code and update documentation when necessary.
- **Testing**: Test your changes thoroughly before submitting a pull request.
- **Issues**: Create an issue before starting work on significant changes.

## 📄 License

This project is licensed under the MIT License.

## 🆘 Support

Need help with Obsidian Recursive Share? We're here to assist you!
If you encounter a bug or have a feature request:

1. Check the [GitHub Issues](https://github.com/Bloeckchengrafik/obsidian-recursive-share/issues) to see if it has
   already been reported.
2. If not, create a new issue with:
    - A clear, descriptive title
    - Detailed steps to reproduce the issue
    - Information about your environment (Obsidian version, OS, etc.)
    - Screenshots if applicable

### 🔍 Troubleshooting Tips

Before reaching out for support, try these troubleshooting steps:

1. **Update the Plugin**: Make sure you're using the latest version
2. **Restart Obsidian**: Many issues can be resolved with a simple restart
3. **Check Console Errors**: Open the developer console (Ctrl+Shift+I) to check for errors
4. **Disable Other Plugins**: Temporarily disable other plugins to check for conflicts
5. **Try in a New Vault**: Test if the issue persists in a clean vault


