use std::cell::Cell;
use web_sys::HtmlInputElement;
use yew::events::InputEvent;
use yew::{html, Callback, Component, Context, Html, Properties, TargetCast};

// Define a thread-local variable to keep track of switch IDs
thread_local! {
    static SWITCH_ID: Cell<usize> = Cell::default();
}

// Function to generate the next switch ID
fn next_switch_id() -> usize {
    SWITCH_ID.with(|cell| cell.replace(cell.get() + 1))
}

// Define the properties for the Switch component
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub label: &'static str,
    pub value: u64,
    pub onchange: Callback<u64>,
}

// Define the Switch component
pub struct Switch {
    id: usize,
}

// Implement the Component trait for the Switch component
impl Component for Switch {
    type Message = ();
    type Properties = Props;

    // Create a new instance of the Switch component
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            id: next_switch_id(),
        }
    }

    // View function to render the Switch component
    fn view(&self, ctx: &Context<Self>) -> Html {
        // Destructure properties
        let Props {
            label,
            value,
            ref onchange,
        } = *ctx.props();

        // Display value (for potential future modifications)
        let display_value = value;

        // Generate a unique ID for the switch element
        let id = format!("switch-{}", self.id);

        // Callback for input event handling
        let oninput = onchange.reform(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            // Convert checked state to u64 (1 for true, 0 for false)
            let input = if input.checked() { 1 } else { 0 };
            input as u64
        });

        // Render the switch component
        html! {
            <label for={id.clone()}>
                <input type="checkbox" {oninput} role="switch" id={label} name={label} checked={if display_value == 0 { false } else { true }} />
                {label}
            </label>
        }
    }
}
