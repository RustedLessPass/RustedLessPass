#![recursion_limit = "256"]

mod app;
mod fingerprintgen;
mod passgen;
mod password_utils;
mod settings;
mod slider;
mod switch;
mod text_input;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
