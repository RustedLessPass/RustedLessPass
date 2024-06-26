/*
    This module contains functions for generating passwords based on various options
    using the LessPass algorithm.

    The `generate_password` function generates a password based on provided parameters.
    The `generate_charset` function generates the character set based on specified options.

    The file also includes unit tests to ensure the correctness of password generation
    under different scenarios.
*/

use lesspass::{self, CharacterSet};

pub struct PasswordOptions {
    pub domain: String,
    pub login: String,
    pub master_password: String,
    pub lowercase: bool,
    pub uppercase: bool,
    pub digits: bool,
    pub symbols: bool,
    pub length: usize,
    pub counter: u32,
}

/*
    Generates a password based on the provided options.

    # Arguments

    * `options` - A `PasswordOptions` struct containing parameters for generating the password.

    # Returns

    A string representing the generated password.
*/
pub fn generate_password(options: PasswordOptions) -> String {
    let salt = lesspass::generate_salt(&options.domain, &options.login, options.counter);
    let entropy = lesspass::generate_entropy(
        &options.master_password,
        &salt,
        lesspass::Algorithm::SHA256,
        100000,
    );
    let charset = generate_charset(
        options.lowercase,
        options.uppercase,
        options.digits,
        options.symbols,
    );
    lesspass::render_password(&entropy, charset, options.length)
}

/*
    Generates a character set based on the provided options.

    # Arguments

    * `lowercase` - A boolean indicating whether to include lowercase characters.
    * `uppercase` - A boolean indicating whether to include uppercase characters.
    * `digits` - A boolean indicating whether to include digits.
    * `symbols` - A boolean indicating whether to include symbols.

    # Returns

    A `CharacterSet` enum representing the generated character set.
*/
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password_all() {
        let options = PasswordOptions {
            domain: "lorem ipsum".to_string(),
            login: "lorem ipsum".to_string(),
            master_password: "lorem ipsum".to_string(),
            lowercase: true,
            uppercase: true,
            digits: true,
            symbols: true,
            length: 16,
            counter: 1,
        };

        let password = generate_password(options);

        assert_eq!(password, "fV1^3lS*'[knImg8");
    }

    #[test]
    fn test_generate_password_lowercase() {
        let options = PasswordOptions {
            domain: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Neque sodales ut etiam sit amet nisl purus in mollis.".to_string(),
            login: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Neque sodales ut etiam sit amet nisl purus in mollis.".to_string(),
            master_password: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Neque sodales ut etiam sit amet nisl purus in mollis.".to_string(),
            lowercase: true,
            uppercase: false,
            digits: false,
            symbols: false,
            length: 35,
            counter: 100,
        };

        let password = generate_password(options);

        assert_eq!(password, "qxwsvaqanocacbfpvucqxphpcdajhjehoty");
    }

    #[test]
    fn test_generate_password_digits_symbols() {
        let options = PasswordOptions {
            domain: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Neque sodales ut etiam sit amet nisl purus in mollis. Eu consequat ac felis donec et odio pellentesque diam volutpat. Mi in nulla posuere sollicitudin. Euismod quis viverra nibh cras. Tristique nulla aliquet enim tortor at auctor urna nunc. Dignissim convallis aenean et tortor at. Turpis egestas pretium aenean pharetra. Sed vulputate odio ut enim. Faucibus et molestie ac feugiat. Donec ultrices tincidunt arcu non sodales neque sodales ut etiam. Donec pretium vulputate sapien nec sagittis aliquam malesuada. Mauris cursus mattis molestie a iaculis. Hendrerit gravida rutrum quisque non. Enim nulla aliquet porttitor lacus luctus accumsan tortor posuere. Et leo duis ut diam quam nulla. Quam lacus suspendisse faucibus interdum posuere lorem. Adipiscing elit ut aliquam purus sit amet. Consectetur adipiscing elit ut aliquam purus sit amet. Erat imperdiet sed euismod nisi porta lorem mollis.".to_string(),
            login: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Neque sodales ut etiam sit amet nisl purus in mollis. Eu consequat ac felis donec et odio pellentesque diam volutpat. Mi in nulla posuere sollicitudin. Euismod quis viverra nibh cras. Tristique nulla aliquet enim tortor at auctor urna nunc. Dignissim convallis aenean et tortor at. Turpis egestas pretium aenean pharetra. Sed vulputate odio ut enim. Faucibus et molestie ac feugiat. Donec ultrices tincidunt arcu non sodales neque sodales ut etiam. Donec pretium vulputate sapien nec sagittis aliquam malesuada. Mauris cursus mattis molestie a iaculis. Hendrerit gravida rutrum quisque non. Enim nulla aliquet porttitor lacus luctus accumsan tortor posuere. Et leo duis ut diam quam nulla. Quam lacus suspendisse faucibus interdum posuere lorem. Adipiscing elit ut aliquam purus sit amet. Consectetur adipiscing elit ut aliquam purus sit amet. Erat imperdiet sed euismod nisi porta lorem mollis.".to_string(),
            master_password: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Neque sodales ut etiam sit amet nisl purus in mollis. Eu consequat ac felis donec et odio pellentesque diam volutpat. Mi in nulla posuere sollicitudin. Euismod quis viverra nibh cras. Tristique nulla aliquet enim tortor at auctor urna nunc. Dignissim convallis aenean et tortor at. Turpis egestas pretium aenean pharetra. Sed vulputate odio ut enim. Faucibus et molestie ac feugiat. Donec ultrices tincidunt arcu non sodales neque sodales ut etiam. Donec pretium vulputate sapien nec sagittis aliquam malesuada. Mauris cursus mattis molestie a iaculis. Hendrerit gravida rutrum quisque non. Enim nulla aliquet porttitor lacus luctus accumsan tortor posuere. Et leo duis ut diam quam nulla. Quam lacus suspendisse faucibus interdum posuere lorem. Adipiscing elit ut aliquam purus sit amet. Consectetur adipiscing elit ut aliquam purus sit amet. Erat imperdiet sed euismod nisi porta lorem mollis.".to_string(),
            lowercase: false,
            uppercase: false,
            digits: true,
            symbols: true,
            length: 27,
            counter: 52,
        };

        let password = generate_password(options);

        assert_eq!(password, ")_*{$<=&>=8:>9):'*>7/83^#:}");
    }
}
