/*
   This module defines a Settings struct for password generation
   and implements methods to load and store settings using local storage.

   The Settings struct holds parameters such as password length, inclusion
   of different character types, and a list of disabled characters.
*/

use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

// Define a struct to hold settings
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Settings {
    pub size: u64,
    pub counter: u64,
    pub lowercase: u64,
    pub uppercase: u64,
    pub numbers: u64,
    pub symbols: u64,
    pub disabled: String,
}

impl Settings {
    const KEY: &'static str = "rustedlesspass.rustedlesspass.settings";

    pub fn load() -> Self {
        LocalStorage::get(Self::KEY).unwrap_or_default()
    }

    pub fn store(&self) {
        let _ = LocalStorage::set(Self::KEY, self);
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            size: 16,
            counter: 1,
            lowercase: 1,
            uppercase: 1,
            numbers: 1,
            symbols: 1,
            disabled: String::new(),
        }
    }
}
