
use wasm_bindgen::prelude::*;

// Expose this function to JavaScript
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
