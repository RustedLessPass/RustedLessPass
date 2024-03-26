use lesspass::{self, CharacterSet};

pub fn generate_password(
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
    let entropy =
        lesspass::generate_entropy(master_password, &salt, lesspass::Algorithm::SHA256, 100000);
    let charset = generate_charset(lowercase, uppercase, digits, symbols);
    return lesspass::render_password(&entropy, charset, length) as String;
}

fn generate_charset(lowercase: bool, uppercase: bool, digits: bool, symbols: bool) -> CharacterSet {
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
    return charset;
}
