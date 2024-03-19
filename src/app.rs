use yew::prelude::*;

use crate::settings::Settings;
use crate::slider::Slider;
// use crate::text_input::TextInput;

pub enum Msg {
    ChangeSettings(Settings),
    // SetPassword(String),
    // RegeneratePassword,
}

// #[derive(Debug, Default)]
pub struct App {
    settings: Settings,
}

impl App {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            settings: Settings::load(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        // unimplemented!();
        match msg {
            Msg::ChangeSettings(settings) => {
                self.settings = settings;
                self.settings.store();
            } // Msg::SetPassword(next_password) => self.password = next_password,
              // Msg::RegeneratePassword => self.password = generate_password(),
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let on_change = ctx.link().callback(Msg::SetPassword);
        // let onclick = ctx.link().callback(|_| Msg::RegeneratePassword);

        let Self { ref settings, .. } = *self;

        macro_rules! settings_callback {
            ($link:expr, $settings:ident; $key:ident as $ty:ty) => {{
                let settings = $settings.clone();
                $link.callback(move |value| {
                    let mut settings = settings.clone();
                    settings.$key = value as $ty;
                    Msg::ChangeSettings(settings)
                })
            }};
            ($link:expr, $settings:ident; $key:ident) => {
                settings_callback!($link, $settings; $key as u64)
            }
        }

        html! {
            <body>
            <nav class="container-fluid">
                <ul>
                <li>
                    <a href="./"><strong>{"RustlessPass"}</strong></a>
                </li>
                </ul>
                <ul>
                <li>
                    <details class="dropdown">
                    <summary role="button" class="secondary">{"Theme"}</summary>
                    <ul dir="rtl">
                        <li><a href="#" data-theme-switcher="auto">{"Auto"}</a></li>
                        <li><a href="#" data-theme-switcher="light">{"Light"}</a></li>
                        <li><a href="#" data-theme-switcher="dark">{"Dark"}</a></li>
                    </ul>
                    </details>
                </li>
                </ul>
            </nav>

            <main class="container">
                <article>
                <div>
                    <hgroup class="title">
                    <h1>{"Stateless Password Manager"}</h1>
                    <p>{"Remember only one master password to access your passwords at any time, on any device, without the need
                        for
                        syncing."}</p>
                    </hgroup>
                    <form>
                    <input type="text" name="website" placeholder="Website" aria-label="Website" autocomplete="website"
                        required=true />
                    <input type="text" name="username" placeholder="Username" aria-label="Username" autocomplete="username"
                        required=true />
                    <input type="password" name="password" placeholder="Password" aria-label="Password"
                        autocomplete="current-password" required=true />
                    <fieldset>
                        <nav>
                        <label for="a-z">
                            <input type="checkbox" role="switch" id="a-z" name="a-z" checked=true />
                            {"a-z"}
                        </label>
                        <label for="A-Z">
                            <input type="checkbox" role="switch" id="A-Z" name="A-Z" checked=true />
                            {"A-Z"}
                        </label>
                        <label for="0-9">
                            <input type="checkbox" role="switch" id="0-9" name="0-9" checked=true />
                            {"0-9"}
                        </label>
                        <label for="%!@">
                            <input type="checkbox" role="switch" id="%!@" name="%!@" checked=true />
                            {"%!@"}
                        </label>
                        </nav>
                        <div class="grid" style="padding: 0rem;">
                        <Slider label="Size" max=100 min=1 onchange={settings_callback!(ctx.link(), settings; size)}
                            value={settings.size} />
                        <Slider label="Counter" max=100 min=1 onchange={settings_callback!(ctx.link(), settings; counter)}
                            value={settings.counter} />
                        </div>

                    </fieldset>
                    <button type="submit" class="contrast">{"Generate and copy"}</button>
                    </form>
                </div>
                </article>
            </main>

            <footer class="container-fluid">
                <small>{"Built with "}<a href="https://rust-lang.org" class="secondary">{"Rust"}</a>{", "}<a
                    href="https://github.com/71/lesspass.rs" class="secondary">{"lesspass.rs"}</a>{", "}<a href="https://yew.rs"
                    class="secondary">{"Yew"}</a>{" and "}<a href="https://picocss.com" class="secondary">{"Pico"}</a>{" • "}
                <a href="https://github.com/M1n-74316D65/RustlessPass" class="secondary">{"Source code"}</a></small>
            </footer>

            <script src="minimal-theme-switcher.js"></script>
            </body>
        }
    }
}
