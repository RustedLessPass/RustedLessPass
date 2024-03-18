use lesspass::{self, CharacterSet};

fn generate_password(
    domain: &str,
    login: &str,
    master_password: &str,
    lowercase: bool,
    uppercase: bool,
    digits: bool,
    symbols: bool,
    length: usize,
    counter: u32,
) -> String {
    let salt = lesspass::generate_salt(domain, login, counter);
    let entropy = lesspass::generate_entropy(master_password, &salt, lesspass::Algorithm::SHA256, 100000);
    let charset = generate_charset(lowercase, uppercase, digits, symbols);
    lesspass::render_password(&entropy, charset, length)
}

fn generate_charset(
    lowercase: bool,
    uppercase: bool,
    digits: bool,
    symbols: bool,
) -> CharacterSet {
    let mut charset = CharacterSet::All;
    if !lowercase {
        charset.remove(CharacterSet::Lowercase);
    }
    if !uppercase {
        charset.remove(CharacterSet::Uppercase);
    }
    if !digits {
        charset.remove(CharacterSet::Numbers);
    }
    if !symbols {
        charset.remove(CharacterSet::Symbols);
    }
    charset
}

// fn main() {
//     let domain = "example.com";
//     let login = "user123";
//     let master_password = "MySecretMasterPassword";
//     let password = generate_password(
//         domain,
//         login,
//         master_password,
//         true,
//         true,
//         true,
//         true,
//         16,
//         1,
//     );
//     println!("{}", password);
// }