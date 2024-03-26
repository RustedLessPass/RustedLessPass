use yew::prelude::*;
// use lesspass::{self, CharacterSet};

use crate::passgen::generate_password;
use log::info;

// use crate::password_input::PasswordInput;
use crate::text_input::TextInput;
use crate::slider::Slider;
use crate::switch::Switch;
use crate::settings::Settings;

pub enum Msg {
    ChangeSettings(Settings),
    SetWebsite(String),
    SetUsername(String),
    SetPassword(String),
    GeneratePassword,
}

#[derive(Debug, Default)]
pub struct App {
    settings: Settings,
    website: String,
    username: String,
    password: String,
    new_password: String,
}

impl App {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            settings: Settings::load(),
            website: String::new(),
            username: String::new(),
            password: String::new(),
            new_password: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeSettings(settings) => {
                self.settings = settings;
                self.settings.store();
            },
            Msg::SetWebsite(next_website) => self.website = next_website,
            Msg::SetUsername(next_username) => self.username = next_username,
            Msg::SetPassword(next_password) => self.password = next_password,
            Msg::GeneratePassword => {
                // fn generate_charset(
                //     lowercase: bool,
                //     uppercase: bool,
                //     digits: bool,
                //     symbols: bool,
                // ) -> CharacterSet {
                //     let mut charset = CharacterSet::All;
                //     if !lowercase {
                //         charset.remove(CharacterSet::Lowercase);
                //     }
                //     if !uppercase {
                //         charset.remove(CharacterSet::Uppercase);
                //     }
                //     if !digits {
                //         charset.remove(CharacterSet::Numbers);
                //     }
                //     if !symbols {
                //         charset.remove(CharacterSet::Symbols);
                //     }
                //     return charset
                // }
                // let salt = lesspass::generate_salt(self.website.as_str(), self.username.as_str(), self.settings.counter as u32);
                // let entropy = lesspass::generate_entropy(self.password.as_str(), &salt, lesspass::Algorithm::SHA256, 100000);
                // let charset = generate_charset(if self.settings.lowercase == 0 { false } else { true }, if self.settings.uppercase == 0 { false } else { true }, if self.settings.numbers == 0 { false } else { true }, if self.settings.symbols == 0 { false } else { true });
                // self.website = self.website.clone();
                // self.username = self.username.clone();
                // self.password = lesspass::render_password(&entropy, charset, self.settings.size as usize) as String;
                info!("Hello before {}", self.password.clone());
                self.new_password = generate_password(self.website.clone().as_str(), self.username.clone().as_str(), self.password.clone().as_str(), if self.settings.lowercase == 0 { false } else { true }, if self.settings.uppercase == 0 { false } else { true }, if self.settings.numbers == 0 { false } else { true }, if self.settings.symbols == 0 { false } else { true }, self.settings.size as usize, self.settings.counter as u32) as String;
                info!("Final {} 2", self.new_password.clone());
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_website_change = ctx.link().callback(Msg::SetWebsite);
        let on_username_change = ctx.link().callback(Msg::SetUsername);
        let on_password_change = ctx.link().callback(Msg::SetPassword);
        let onclick = ctx.link().callback(|_| Msg::GeneratePassword);

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
                    <TextInput value={self.website.clone()} input_type={"text"} name={"Website"} autocomplete={"off"} on_change={on_website_change}/>
                    <TextInput value={self.username.clone()} input_type={"text"} name={"Username"} autocomplete={"email,username"} on_change={on_username_change}/>
                    <TextInput value={self.password.clone()} input_type={"password"} name={"Password"} autocomplete={"current-password"} on_change={on_password_change}/>
                    <fieldset>
                        <nav>
                            <Switch label="Lower Case" onchange={settings_callback!(ctx.link(), settings; lowercase)} value={settings.lowercase.clone()} />
                            <Switch label="Upper Case" onchange={settings_callback!(ctx.link(), settings; uppercase)} value={settings.uppercase.clone()} />
                            <Switch label="Numbers" onchange={settings_callback!(ctx.link(), settings; numbers)} value={settings.numbers.clone()} />
                            <Switch label="Symbols" onchange={settings_callback!(ctx.link(), settings; symbols)} value={settings.symbols.clone()} />
                        </nav>
                        <div class="grid" style="padding: 0rem;">
                        <Slider label="Size" max=35 min=1 onchange={settings_callback!(ctx.link(), settings; size)}
                            value={settings.size.clone()} />
                        <Slider label="Counter" max=100 min=1 onchange={settings_callback!(ctx.link(), settings; counter)}
                            value={settings.counter.clone()} />
                        </div>

                    </fieldset>
                    <button type="submit" class="contrast" {onclick}>{"Generate and copy"}</button>
                    <p>{"Your generated password is: "}{&self.new_password}</p>
                    <p>{"Your generated password will be copied to your clipboard."}</p>
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
