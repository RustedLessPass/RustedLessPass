use std::cell::Cell; // Import Cell for thread-local mutable memory
use web_sys::HtmlInputElement; // Import HtmlInputElement for input element manipulation
use yew::events::InputEvent; // Import InputEvent for handling input events
use yew::{html, Callback, Component, Context, Html, Properties, TargetCast}; // Import necessary items from Yew library

// Define a thread-local static variable to hold the slider ID
thread_local! {
    static SLIDER_ID: Cell<usize> = Cell::default();
}

// Function to generate next slider ID
fn next_slider_id() -> usize {
    SLIDER_ID.with(|cell| cell.replace(cell.get() + 1))
}

// Define properties for the Slider component
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub label: &'static str,     // Label for the slider
    pub value: u64,              // Current value of the slider
    pub onchange: Callback<u64>, // Callback function for when the slider value changes
    pub min: u64,                // Minimum value of the slider
    pub max: u64,                // Maximum value of the slider
}

// Define the Slider component
pub struct Slider {
    id: usize, // Unique ID for the slider
}

// Implement the Component trait for Slider
impl Component for Slider {
    type Message = (); // Define the message type for the component
    type Properties = Props; // Define the properties type for the component

    // Method to create a new instance of the Slider component
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            id: next_slider_id(), // Generate a new unique ID for the slider
        }
    }

    // Method to render the view of the Slider component
    fn view(&self, ctx: &Context<Self>) -> Html {
        // Destructure properties from context
        let Props {
            label,
            value,
            ref onchange,
            min,
            max,
        } = *ctx.props();

        let display_value = value; // Assign the current value of the slider

        let id = format!("slider-{}", self.id); // Generate unique ID for the input element

        // Callback function for handling input events
        let oninput = onchange.reform(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            input.value_as_number() as u64
        });

        // Generate HTML for the Slider component
        html! {
            <div class="slider">
                <label for={id.clone()} class="slider__label">{ label }{": "}{ display_value }</label>
                <input type="range"
                    value={value.to_string()} // Convert value to string
                    {id} // Set the ID of the input element
                    class="slider__input"
                    min={min.to_string()} max={max.to_string()} step={"1"} // Set min, max, and step attributes
                    {oninput} // Set the oninput event handler
                />
            </div>
        }
    }
}
