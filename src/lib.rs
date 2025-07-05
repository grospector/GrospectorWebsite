use wasm_bindgen::prelude::*;
use web_sys::console;

mod components;
mod services;
mod types;
mod utils;

use components::app::App;

// Initialize the WASM application
#[wasm_bindgen(start)]
pub fn run_app() {
    // Set up logging - only show errors
    wasm_logger::init(wasm_logger::Config::new(log::Level::Error));
    console::log_1(&"Bitcoin Wealth Comparison App Starting...".into());

    // Mount the Yew app
    yew::Renderer::<App>::new().render();
}

// Called when the wasm module is instantiated
#[wasm_bindgen]
pub fn main() {
    run_app();
}
