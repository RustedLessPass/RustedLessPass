// This attribute increases the recursion limit to accommodate the Yew framework's requirements.
#![recursion_limit = "256"]

// Importing module declarations
mod app; // Module for the main application component
mod fingerprintgen; // Module for fingerprint generation functionality
mod passgen; // Module for password generation functionality
mod settings; // Module for application settings
mod slider; // Module for slider component
mod switch; // Module for switch component
mod text_input; // Module for text input component

use app::App; // Importing the main application component

// Entry point of the program
fn main() {
    // Create a new renderer instance for the App component and initiate rendering
    yew::Renderer::<App>::new().render();
}
