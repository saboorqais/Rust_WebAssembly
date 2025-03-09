use wasm_bindgen::prelude::*;

// Expose functions to JavaScript
#[wasm_bindgen]
pub fn format_text(text: &str, tag: &str) -> String {
    format!("<{}>{}</{}>", tag, text, tag)
}

