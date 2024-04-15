/*
    This module contains utility functions for managing password generation and display.

    The `update_disabled_characters` function updates the list of disabled characters based on the provided settings.
    The `update_show_state` function updates the show state based on the input parameters.
*/

use wasm_bindgen_futures::spawn_local;

use crate::passgen::generate_password;
use crate::passgen::PasswordOptions;
use crate::settings::Settings;

/*
    Updates the list of disabled characters based on the provided settings.

    Arguments:
    - `settings`: A reference to the `Settings` struct containing the user's preferences.

    Returns:
    A `String` containing the updated list of disabled characters.
*/
pub fn update_disabled_characters(settings: &Settings) -> String {
    if settings.lowercase == 1
        && settings.uppercase == 0
        && settings.numbers == 0
        && settings.symbols == 0
    {
        "a-z".to_string()
    } else if settings.lowercase == 0
        && settings.uppercase == 1
        && settings.numbers == 0
        && settings.symbols == 0
    {
        return "A-Z".to_string();
    } else if settings.lowercase == 0
        && settings.uppercase == 0
        && settings.numbers == 1
        && settings.symbols == 0
    {
        return "0-9".to_string();
    } else if settings.lowercase == 0
        && settings.uppercase == 0
        && settings.numbers == 0
        && settings.symbols == 1
    {
        return "%!@".to_string();
    } else {
        return "".to_string();
    }
}

/*
    Updates the show state based on the input parameters and generates the password.

    Arguments:
    - `show`: An unsigned 8-bit integer indicating the show state.
    - `website`: A string slice representing the website domain.
    - `username`: A string slice representing the username.
    - `password`: A string slice representing the master password.
    - `settings`: A reference to the `Settings` struct containing the user's preferences.
    - `new_password`: A string slice representing the new password.

    Returns:
    A tuple containing the updated show state and the new password.
*/
pub fn update_show_state(
    show: u8,
    website: &str,
    username: &str,
    password: &str,
    settings: &Settings,
    new_password: &str,
) -> (u8, String) {
    match show {
        // If 'show' is 0
        0 => {
            let password_options = PasswordOptions {
                domain: website.to_string(),
                login: username.to_string(),
                master_password: password.to_string(),
                lowercase: settings.lowercase != 0,
                uppercase: settings.uppercase != 0,
                digits: settings.numbers != 0,
                symbols: settings.symbols != 0,
                length: settings.size as usize,
                counter: settings.counter as u32,
            };

            let new_password = generate_password(password_options);

            let cloned_new_password = new_password.clone();
            spawn_local(async move {
                let window = web_sys::window().expect("window");
                let nav = window.navigator().clipboard();
                if let Some(a) = nav {
                    let p = a.write_text(&cloned_new_password);
                    let _result = wasm_bindgen_futures::JsFuture::from(p)
                        .await
                        .expect("clipboard populated");
                }
            });
            (1, new_password.to_string())
        }
        1 => (2, new_password.to_string()),
        _ => (1, new_password.to_string()),
    }
}
