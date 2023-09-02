![Alt Text](https://github.com/OlekVano/password-manager/blob/master/images/main.png)

A simple command-line password manager written in Rust. It allows you to securely store and manage your passwords using strong encryption. Here's a brief overview of its features and how to use it:

## Features

- Create a master password for your password vault.
- Add, remove, edit, and retrieve passwords.
- Passwords are encrypted using strong encryption algorithms.
- Copy passwords to the clipboard for easy use.
- Command-line interface for quick and efficient password management.

## Usage

1. **First Run**: When you run the application for the first time, you will be prompted to create a master password. This password is used to encrypt and decrypt your stored passwords.

2. **Main Menu**: After setting up the master password, you'll enter the main menu. Here, you can perform various password management tasks using commands like `add`, `rem`, `get`, `edit`, and `exit`.

3. **Commands**:
   - `add <name>`: Add a new password entry.
   - `rem <name>`: Remove an existing password entry.
   - `get <name>`: Retrieve a password and copy it to the clipboard.
   - `edit <name>`: Edit an existing password entry.
   - `exit`: Exit the application.
   - `help`: Show available commands.

4. **Security**: Passwords are stored encrypted, ensuring your data's security even if someone gains access to the save file.

## Dependencies

- [ansi_term](https://crates.io/crates/ansi_term): For colored text output.
- [clipboard](https://crates.io/crates/clipboard): For managing clipboard operations.
- [magic-crypt](https://crates.io/crates/magic-crypt): For encryption and decryption.
- [rpassword](https://crates.io/crates/rpassword): For securely reading passwords.
- [serde_json](https://crates.io/crates/serde_json): For JSON serialization and deserialization.

## Getting Started

1. Clone the repository to your local machine.
2. Build the project using Rust's Cargo: `cargo build --release`
3. Run the application: `cargo run`

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributions

Contributions are welcome! If you have any suggestions, bug reports, or feature requests, please open an issue or create a pull request.
