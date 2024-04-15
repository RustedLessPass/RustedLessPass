/*
    This module defines the Yew component for the RustedLessPass application.

    It includes:
        - The definition of the `Msg` enum representing different messages.
        - The `App` struct representing the application state.
        - Implementation of the `Component` trait for the `App` struct, providing methods for
        creating, updating, and viewing the component.

    The `App` component serves as the core controller of the password manager application.
    It manages the application state, handles user interactions, and renders the user interface.
*/

use crate::fingerprintgen::fingerprint_calculate;
use crate::password_utils::{update_disabled_characters, update_show_state};
use crate::settings::Settings;
use crate::slider::Slider;
use crate::switch::Switch;
use crate::text_input::TextInput;
use yew::prelude::*;

pub enum Msg {
    ChangeSettings(Settings),
    SetWebsite(String),
    SetUsername(String),
    SetPassword(String),
    GeneratePassword,
    ShowInputPassword,
}

pub struct App {
    settings: Settings,
    website: String,
    username: String,
    password: String,
    new_password: String,
    fingerprint: Vec<String>,
    show: u8,
    show_input_password: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            settings: Settings::load(),
            website: String::new(),
            username: String::new(),
            password: String::new(),
            new_password: "Generate and copy".to_string(),
            fingerprint: fingerprint_calculate(""),
            show: 0,
            show_input_password: false,
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeSettings(settings) => {
                self.settings = settings.clone();
                self.settings.disabled = update_disabled_characters(&settings);
                self.settings.store();
                self.show = 0;
            }
            Msg::SetWebsite(next_website) => {
                self.website = next_website;
                self.show = 0;
            }
            Msg::SetUsername(next_username) => {
                self.username = next_username;
                self.show = 0;
            }
            Msg::SetPassword(next_password) => {
                self.password = next_password;
                self.fingerprint = fingerprint_calculate(self.password.clone().as_str());
                self.show = 0;
            }
            Msg::GeneratePassword => {
                (self.show, self.new_password) = update_show_state(
                    self.show,
                    &self.website,
                    &self.username,
                    &self.password,
                    &self.settings,
                    &self.new_password,
                );
            }

            Msg::ShowInputPassword => {
                self.show_input_password = !self.show_input_password;
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_website_change = ctx.link().callback(Msg::SetWebsite);
        let on_username_change = ctx.link().callback(Msg::SetUsername);
        let on_password_change = ctx.link().callback(Msg::SetPassword);
        let on_password_click = ctx.link().callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::ShowInputPassword
        });
        let on_submit = ctx.link().callback(|e: SubmitEvent| {
            e.prevent_default();
            Msg::GeneratePassword
        });

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
                    <a href="./"><img src="assets/icons/maskable_icon_x48.png" alt="Description of the image" width="37.7048437335240"
                        height="37.7048437335240"/><strong>{"RustedLessPass"}</strong></a>
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
                    <form onsubmit={on_submit}>
                    <TextInput value={self.website.clone()} input_type={"text"} name={"Website"} autocomplete={"off"}
                        on_change={on_website_change} />
                    <TextInput value={self.username.clone()} input_type={"text"} name={"Username"} autocomplete={"email,username"}
                        on_change={on_username_change} />
                    <fieldset role="group">
                        <TextInput value={self.password.clone()} input_type={if self.show_input_password {"text"} else {"password"}}
                        name={"Password"} autocomplete={"current-password"} on_change={on_password_change} />
                        <button
                            style="white-space: nowrap;margin-left: 0rem; padding-left: 0.5rem; padding-right: 0.5rem; align-self: center;"
                            onclick={on_password_click}>
                            <i class={match self.fingerprint.first() { Some(s)=> format!("fa fa-fw {}", s),
                            None => String::new(),
                            }} style="margin-right: 0.2rem;"></i>
                            <i class={match self.fingerprint.get(1) { Some(s)=> format!("fa fa-fw {}", s),
                            None => String::new(),
                            }} style="margin-left: 0.2rem;margin-right: 0.2rem;p"></i>
                            <i class={match self.fingerprint.last() { Some(s)=> format!("fa fa-fw {}", s),
                            None => String::new(),
                            }} style="margin-left: 0.2rem;"></i>
                        </button>
                    </fieldset>
                    <fieldset>
                        <nav>
                        <Switch label="a-z" onchange={settings_callback!(ctx.link(), settings; lowercase)}
                            value={settings.lowercase} value_disabled={self.settings.disabled.clone() == "a-z"} />
                        <Switch label="A-Z" onchange={settings_callback!(ctx.link(), settings; uppercase)}
                            value={settings.uppercase} value_disabled={self.settings.disabled.clone() == "A-Z"} />
                        <Switch label="0-9" onchange={settings_callback!(ctx.link(), settings; numbers)}
                            value={settings.numbers} value_disabled={self.settings.disabled.clone() == "0-9"} />
                        <Switch label="%!@" onchange={settings_callback!(ctx.link(), settings; symbols)}
                            value={settings.symbols} value_disabled={self.settings.disabled.clone() == "%!@"} />
                        </nav>
                        <div class="grid" style="padding: 0rem;">
                        <Slider label="Size" max=35 min=1 onchange={settings_callback!(ctx.link(), settings; size)}
                            value={settings.size} />
                        <Slider label="Counter" max=100 min=1 onchange={settings_callback!(ctx.link(), settings; counter)}
                            value={settings.counter} />
                        </div>
                    </fieldset>
                    <button type="submit" class="contrast">{if self.show == 0 {"Generate and copy"} else if self.show == 1
                        {"**************"} else {self.new_password.as_str()}}</button>
                    </form>
                </div>
                </article>
            </main>

            <footer class="container-fluid">
                <small>{"Built with "}<a href="https://rust-lang.org" class="secondary">{"Rust"}</a>{", "}<a
                    href="https://github.com/71/lesspass.rs" class="secondary">{"lesspass.rs"}</a>{", "}<a href="https://yew.rs"
                    class="secondary">{"Yew"}</a>{" and "}<a href="https://picocss.com" class="secondary">{"Pico"}</a>{" • "}
                <a href="https://github.com/RustedLessPass/RustedLessPass" class="secondary">{"Source code"}</a></small>
            </footer>
            <script src="assets/minimal-theme-switcher.js"></script>
            </body>
        }
    }
}
