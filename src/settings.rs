use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Settings {
    pub size: u64,
    pub counter: u64,
    pub lowercase: u64,
    pub uppercase: u64,
    pub numbers: u64,
    pub symbols: u64,
}
impl Settings {
    const KEY: &'static str = "yew.boids.settings";

    pub fn load() -> Self {
        LocalStorage::get(Self::KEY).unwrap_or_default()
    }

    // pub fn remove() {
    //     LocalStorage::delete(Self::KEY);
    // }

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
        }
    }
}
