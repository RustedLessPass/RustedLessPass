#![recursion_limit = "256"]

mod text_input;

mod app;
mod slider;
mod switch;
mod settings;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
