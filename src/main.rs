#![recursion_limit = "256"]

mod app;
mod passgen;
mod fingerprintgen;
mod settings;
mod slider;
mod switch;
mod text_input;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
