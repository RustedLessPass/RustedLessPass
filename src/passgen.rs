use lesspass::{self, CharacterSet};

/// Generates a password based on given parameters.
///
/// # Arguments
///
/// * `domain` - A string slice representing the domain name associated with the password.
/// * `login` - A string slice representing the login associated with the password.
/// * `master_password` - A string slice representing the master password used for generation.
/// * `lowercase` - A boolean indicating whether lowercase characters are included.
/// * `uppercase` - A boolean indicating whether uppercase characters are included.
/// * `digits` - A boolean indicating whether digits are included.
/// * `symbols` - A boolean indicating whether symbols are included.
/// * `length` - The length of the password to be generated.
/// * `counter` - A counter value for generating the salt.
///
/// # Returns
///
/// A string representing the generated password.
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
    lesspass::render_password(&entropy, charset, length)
}

/// Generates a character set based on given parameters.
///
/// # Arguments
///
/// * `lowercase` - A boolean indicating whether lowercase characters are included.
/// * `uppercase` - A boolean indicating whether uppercase characters are included.
/// * `digits` - A boolean indicating whether digits are included.
/// * `symbols` - A boolean indicating whether symbols are included.
///
/// # Returns
///
/// A CharacterSet enum representing the generated character set.
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
    charset
}
