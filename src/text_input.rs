use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::prelude::*;

// Define the properties required by the TextInput component
#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: String,
    pub input_type: String,
    pub name: String,
    pub autocomplete: String,
    pub on_change: Callback<String>,
}

// Extract the value from the input event
fn get_value_from_input_event(e: InputEvent) -> String {
    // Convert the generic event to a specific type
    let event: Event = e.dyn_into().unwrap_throw();
    // Get the target of the event, which should be an input element
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    // Return the value of the input element
    target.value()
}

/// Controlled Text Input Component
#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    // Destructure the properties for easier access
    let Props {
        value,
        input_type,
        name,
        autocomplete,
        on_change,
    } = props.clone();

    // Reform the on_change callback to pass the input event's value
    let oninput =
        on_change.reform(|input_event: InputEvent| get_value_from_input_event(input_event));

    // Render the HTML input element
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
