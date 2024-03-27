# RustlessPass

RustlessPass is a password manager built with Rust, utilizing the lesspass.rs library for password generation and the Yew framework for building the web interface.

## Installation

To run RustlessPass locally, follow these steps:

1. Clone the repository:
```bash
git clone https://github.com/M1n-74316D65/RustlessPass.git
git submodule update --init --recursive
```
2. Navigate to the project directory:
```bash
cd RustlessPass
```
3. Install dependencies:
```bash
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```
4. Serve the application using the trunk development server:
```bash
trunk serve --open
```

This will open the RustlessPass application in your default web browser.

## Usage

RustlessPass provides a simple and secure way to generate and manage passwords. Users only need to remember their master password, and RustlessPass will generate unique passwords for each site or service based on user-specific inputs.

To use RustlessPass:

1. Enter your master password.
2. Provide the website name and login information.
3. RustlessPass will generate a unique password for that site.

You can then use the generated password when signing up or logging into websites, without the need to store or remember individual passwords.

## Contributing

Contributions to RustlessPass are welcome! If you find any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request.

## Special Thanks

Sincere gratitude to:

- [Pico CSS](https://picocss.com)
- [lesspass](https://github.com/lesspass/lesspass)
- [lesspass.rs](https://github.com/71/lesspass.rs)
- [Yew Framework](https://yew.rs)
- [Rust Programming Language](https://rust-lang.org)

## License

RustlessPass is licensed under the [GPL-3.0 License](LICENSE).
