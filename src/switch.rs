/*
   This module defines a Switch component for Yew, which is used to create
   customizable switches with labels, values, and callback functions.

   The Switch component utilizes thread-local mutable memory for generating
   unique IDs and integrates with HTML input elements for input manipulation.
*/

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
    pub value_disabled: bool,
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            label,
            value,
            value_disabled,
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
                <input type="checkbox" {oninput} role="switch" id={label} name={label} checked={display_value != 0} disabled={value_disabled}/>
                {label}
            </label>
        }
    }
}
