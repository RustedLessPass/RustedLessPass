use std::cell::Cell;

use web_sys::HtmlInputElement;
use yew::events::InputEvent;
use yew::{html, Callback, Component, Context, Html, Properties, TargetCast};

thread_local! {
    static SWITCH_ID: Cell<usize> = Cell::default();
}
fn next_switch_id() -> usize {
    SWITCH_ID.with(|cell| cell.replace(cell.get() + 1))
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub label: &'static str,
    pub value: u64,
    pub onchange: Callback<u64>,
}

pub struct Switch {
    id: usize,
}
impl Component for Switch {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            id: next_switch_id(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        unimplemented!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            label,
            value,
            ref onchange,
        } = *ctx.props();

        let display_value = value;

        let id = format!("switch-{}", self.id);

        let oninput = onchange.reform(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let input = if input.checked() { 1 } else { 0 };
            input as u64
        });

        html! {
            <label for={id.clone()}>
                <input type="checkbox" {oninput} role="switch" id={label} name={label} checked={if display_value == 0 { false } else { true }} />
                {label}
            </label>
        }
    }
}
