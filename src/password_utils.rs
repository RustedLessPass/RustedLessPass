use wasm_bindgen_futures::spawn_local;

use crate::passgen::generate_password;
use crate::passgen::PasswordOptions;
use crate::settings::Settings;

// Function to update disabled characters based on settings
pub fn update_disabled_characters(settings: &Settings) -> String {
    // Check if only lowercase characters are enabled
    if settings.lowercase == 1
        && settings.uppercase == 0
        && settings.numbers == 0
        && settings.symbols == 0
    {
        "a-z".to_string()
    }
    // Check if only uppercase characters are enabled
    else if settings.lowercase == 0
        && settings.uppercase == 1
        && settings.numbers == 0
        && settings.symbols == 0
    {
        return "A-Z".to_string();
    }
    // Check if only numbers are enabled
    else if settings.lowercase == 0
        && settings.uppercase == 0
        && settings.numbers == 1
        && settings.symbols == 0
    {
        return "0-9".to_string();
    }
    // Check if only symbols are enabled
    else if settings.lowercase == 0
        && settings.uppercase == 0
        && settings.numbers == 0
        && settings.symbols == 1
    {
        return "%!@".to_string();
    }
    // Return an empty string if no characters are enabled
    else {
        return "".to_string();
    }
}

// Function to update show state based on settings
pub fn update_show_state(
    show: u8,
    website: &str,
    username: &str,
    password: &str,
    settings: &Settings,
    new_password: &str,
) -> (u8, String) {
    // Match the value of 'show'
    match show {
        // If 'show' is 0
        0 => {
            // Generate new password based on settings
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

            // Clone the new password for asynchronous use
            let cloned_new_password = new_password.clone();
            // Spawn a local asynchronous task to interact with the clipboard
            spawn_local(async move {
                let window = web_sys::window().expect("window");
                let nav = window.navigator().clipboard();
                // match nav {
                //     Some(a) => {
                //         let p = a.write_text(&cloned_new_password);
                //         let _result = wasm_bindgen_futures::JsFuture::from(p)
                //             .await
                //             .expect("clipboard populated");
                //     }
                //     None => {}
                // };
                if let Some(a) = nav {
                    let p = a.write_text(&cloned_new_password);
                    let _result = wasm_bindgen_futures::JsFuture::from(p)
                        .await
                        .expect("clipboard populated");
                }
            });
            // Return the updated show state and the new password
            (1, new_password.to_string())
        }
        // If 'show' is 1
        1 => (2, new_password.to_string()),
        // If 'show' is any other value
        _ => (1, new_password.to_string()),
    }
}
