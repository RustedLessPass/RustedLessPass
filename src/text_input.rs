/*
   This module defines a controlled TextInput component for Yew, which is used to create
   customizable text input fields with controlled behavior.

   The TextInput component utilizes wasm_bindgen and web_sys to handle input events and
   interact with HTML input elements. It also utilizes Yew for rendering and managing
   the component lifecycle.
*/

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: String,
    pub input_type: String,
    pub name: String,
    pub autocomplete: String,
    pub on_change: Callback<String>,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let Props {
        value,
        input_type,
        name,
        autocomplete,
        on_change,
    } = props.clone();

    let oninput =
        on_change.reform(|input_event: InputEvent| get_value_from_input_event(input_event));

    html! {
        <input
            type={input_type}
            name={name.clone()}
            placeholder={name.clone()}
            aria-label={name}
            autocomplete={autocomplete}
            required=true
            value={value}
            oninput={oninput}
        />
    }
}
