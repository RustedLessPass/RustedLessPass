# RustedLessPass 

**Tired of juggling weak passwords?** RustedLessPass empowers you to manage your login credentials securely and conveniently. Built with Rust and WebAssembly, RustedLessPass offers a platform-independent solution for your password needs.

## **Key Features:**

- **LessPass Compatibility:** Enjoy a familiar user experience.
- **WebAssembly Power:** Run RustedLessPass directly in your web browser, eliminating server-side dependencies and keeping your data entirely under your control.
- **Cross-Platform Access:** Manage your passwords from any device with a modern web browser, no matter the operating system.
- **Open-Source Transparency:** Benefit from the power and security of open-source development, allowing for community contributions and independent audits.

## **Installation:**

- **Recommended:** Use the Progressive Web App (PWA) version for seamless browser and device integration.
- **Alternative:** Accessible installers are available for Firefox and traditional PC installs [link to installers](https://github.com/RustedLessPass/RustedLessPass/releases/latest).

## Screenshot

![screenshot](https://github.com/RustedLessPass/RustedLessPass/assets/54779580/68a67f3c-38f5-4e00-a7b1-28dfb92ad6f2)

## Usage

RustedLessPass provides a simple and secure way to generate and manage passwords. Users only need to remember their master password, and RustedLessPass will generate unique passwords for each site or service based on user-specific inputs.

To use RustedLessPass:

1. Enter your master password.
2. Provide the website name and login information.
3. RustedLessPass will generate a unique password for that site.

You can then use the generated password when signing up or logging into websites, without the need to store or remember individual passwords.

## **Building Locally (Optional):**

This section is intended for developers who want to contribute or run the application locally.

1. Clone the repository:
```bash
git clone https://github.com/RustedLessPass/RustedLessPass.git
```
2. Navigate to the project directory:
```bash
cd RustedLessPass
```
3. Install dependencies:
```bash
git submodule update --init --recursive
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```
4. Serve the application using the trunk development server:
```bash
trunk serve --open
```

This will open the RustedLessPass application in your default web browser.

## Contributing

Contributions to RustedLessPass are welcome! If you find any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request.

## Special Thanks

Sincere gratitude to:

- [Pico CSS](https://picocss.com)
- [Lesspass](https://github.com/lesspass/lesspass)
- [lesspass.rs](https://github.com/71/lesspass.rs)
- [Tauri](https://tauri.app/)
- [Yew Framework](https://yew.rs)
- [Rust Programming Language](https://rust-lang.org)

## License

RustedLessPass is licensed under the [GPL-3.0 License](LICENSE).
