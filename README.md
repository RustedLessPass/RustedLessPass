# RustedLessPass 

A password manager built in Rust, compatible with LessPass.

Powered by Yew for WebAssembly compatibility, it runs directly in your web browser, so there's no need for any server computation—only for delivering the app.

Inspired by LessPass, it follows the same principles but works independently.

This means you can generate strong passwords without relying on external servers.

## Screenshot

![imagen](https://github.com/RustedLessPass/RustedLessPass/assets/54779580/db64b347-6875-4886-8f79-c6d7c31bf1c0)

## Usage

RustedLessPass provides a simple and secure way to generate and manage passwords. Users only need to remember their master password, and RustedLessPass will generate unique passwords for each site or service based on user-specific inputs.

To use RustedLessPass:

1. Enter your master password.
2. Provide the website name and login information.
3. RustedLessPass will generate a unique password for that site.

You can then use the generated password when signing up or logging into websites, without the need to store or remember individual passwords.

## Installation

To run RustedLessPass locally, follow these steps:

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
- [lesspass](https://github.com/lesspass/lesspass)
- [lesspass.rs](https://github.com/71/lesspass.rs)
- [Yew Framework](https://yew.rs)
- [Rust Programming Language](https://rust-lang.org)

## License

RustedLessPass is licensed under the [GPL-3.0 License](LICENSE).
