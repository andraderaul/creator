# Creator

[![Code Quality](https://github.com/andraderaul/creator/actions/workflows/quality.yml/badge.svg)](https://github.com/andraderaul/creator/actions/workflows/quality.yml) [![Release](https://github.com/andraderaul/creator/actions/workflows/release.yml/badge.svg)](https://github.com/andraderaul/creator/actions/workflows/release.yml) [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## About

**Creator** is a command-line tool designed to help maintain a consistent folder structure in your projects, particularly tailored for React Native applications. It allows you to define and enforce a folder structure pattern through a `config.json` file.

## 📝 Table of Contents

- [Features](#🚧-features)
- [Downloading Artifacts](#🧱-downloading-artifacts)
- [Installation](#🚚-installation)
- [Usage](#🎙️-usage)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgements](#acknowledgements)
- [Contact](#📇-contact)

## 🚧 Features

This is the current roadmap for **Creator**:

- [x] **Folder Structure Definition**: Define and customize your project's folder structure using a `config.json` file.

- [x] **Source Directory Definition**: Define and customize where your project's folder structure will be created.

- [ ] **Generate Code Definition**: Generate code based on the config file.

- [ ] **CLI Interface**: Use a simple and intuitive command-line interface to create:
  - [x] new features
  - [x] new core
  - [x] new application
  - [ ] new subdirectory based on the config file

## 🧱 Downloading Artifacts

Currently, you can download pre-built binaries for this project from the [Releases](https://github.com/andraderaul/creator/actions/workflows/release.yml) page. Select the appropriate version and download the binary for your platform.

## 🚚 Installation

After downloading the binary, you can follow these steps to install and use the CLI:

1. **Linux:**

   ```bash
   chmod +x creator
   sudo mv creator /usr/local/bin/
   ```

2. **macOS:**
   ```bash
   chmod +x creator
   mv creator /usr/local/bin/
   ```

## 🎙️ Usage

> TODO: include screenshots or gif

You can see all the CLI commands by running the following command.

```bash
creator --help
```

## 👯 Contributing

We welcome contributions to make **Creator** even better. If you have suggestions, bug reports, or want to contribute code, follow these steps:

1. Assuming that you have

   - [x] Installed and configured [git](https://git-scm.com/downloads)
   - [x] Installed [Rust](https://www.rust-lang.org/tools/install)
   - [x] Setup an [SSH key](https://support.atlassian.com/bitbucket-cloud/docs/set-up-an-ssh-key/)

2. To run **Creator** on your machine in developer mode, follow these steps:

   ```bash
   # Clone the repository
   git clone https://github.com/andraderaul/creator.git

   # Change into the project directory
   cd creator

   # Build the project
   cargo build

   # Run the project
   cargo run -- new-feature my_feature
   ```

3. Fork the project.
4. Create your feature branch: `git checkout -b feature/my-feature`.
5. Commit your changes: `git commit -am 'Add new feature'`.
6. Push to the branch: `git push origin feature/my-feature`.
7. Submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

<!--If your project was inspired by others, or if you used external libraries, tools, or resources, acknowledge them here.-->

> TODO: Improve this section

## 📇 Contact

For questions, feedback, or issues, feel free to reach out:

- Email: theandraderaul@gmail.com
