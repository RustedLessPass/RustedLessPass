extern crate zxcvbn;

use yew::prelude::*;
use zxcvbn::zxcvbn;

use crate::password::generate_password;
use crate::text_input::TextInput;

pub enum Msg {
    // SetPassword(String),
    // RegeneratePassword,
}

#[derive(Debug, Default)]
pub struct App {
    password: String,
}

impl App {
    fn get_estimate(&self) -> Option<u8> {
        zxcvbn(&self.password, &[])
            .ok()
            .map(|estimate| estimate.score())
    }

    fn redout_top_row_text(&self) -> String {
        if self.password.is_empty() {
            return "Provide a password".to_string();
        }
        let estimate_text = match self.get_estimate().unwrap_or(0) {
            0 => "That's a password?",
            1 => "You can do a lot better.",
            2 => "Meh",
            3 => "Good",
            _ => "Great!",
        };
        format!("Complexity = {estimate_text}")
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
            // Msg::SetPassword(next_password) => self.password = next_password,
            // Msg::RegeneratePassword => self.password = generate_password(),
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let on_change = ctx.link().callback(Msg::SetPassword);
        // let onclick = ctx.link().callback(|_| Msg::RegeneratePassword);
        html! {
            // <main>
            //     <div class="entry">
            //         <div>
            //             {"Aqui va texto:"}
            //             <div class="footnote">
            //                 {"(Will show in clear text)"}
            //             </div>
            //         </div>
            //         <div>
            //             <TextInput {on_change} value={self.password.clone()} />
            //         </div>
            //     </div>
            //     <div class="readout">
            //         <div>
            //             {self.redout_top_row_text()}
            //         </div>
            //         <button {onclick}>
            //             {"Generate new password *"}
            //         </button>
            //         <div class="footnote">
            //             {"* Note: generated passwords are not actually cryptographically secure"}
            //         </div>
            //     </div>
            // </main>
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
                    <p>{"Remember only one master password to access your passwords at any time, on any device, without the need for
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
                        <div>
                            <label>
                            {"Size: "}<output id="sizeOutput">{"16"}</output>
                            <input type="range" id="sizeRange" value="16" min="1" max="100"
                                oninput= "sizeOutput.value = sizeRange.value"/>
                            </label>
                        </div>
                        <div>
                            <label>
                            {"Counter: "}<output id="counterOutput">{"1"}</output>
                            <input type="range" id="counterRange" value="1" min="1" max="100"
                                oninput="counterOutput.value = counterRange.value"/>
                            </label>
                        </div>
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

            <script src="ui/js/minimal-theme-switcher.js"></script>
            </body>
        }
    }
}
