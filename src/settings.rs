use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

// Define a struct to hold settings
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Settings {
    pub size: u64,      // Length of generated password
    pub counter: u64,   // Number of passwords generated
    pub lowercase: u64, // Include lowercase characters
    pub uppercase: u64, // Include uppercase characters
    pub numbers: u64,   // Include numeric characters
    pub symbols: u64,   // Include symbolic characters
}

impl Settings {
    // Define a constant key for storage
    const KEY: &'static str = "yew.boids.settings";

    // Load settings from local storage or return default
    pub fn load() -> Self {
        LocalStorage::get(Self::KEY).unwrap_or_default()
    }

    // Store settings to local storage
    pub fn store(&self) {
        let _ = LocalStorage::set(Self::KEY, self);
    }
}

impl Default for Settings {
    // Provide a default implementation
    fn default() -> Self {
        Self {
            size: 16,     // Default password length
            counter: 1,   // Default number of passwords
            lowercase: 1, // Include lowercase characters by default
            uppercase: 1, // Include uppercase characters by default
            numbers: 1,   // Include numeric characters by default
            symbols: 1,   // Include symbolic characters by default
        }
    }
}
