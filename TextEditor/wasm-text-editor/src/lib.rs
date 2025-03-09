use wasm_bindgen::prelude::*;
use pulldown_cmark::{Parser, Options, html};
// Expose functions to JavaScript
#[wasm_bindgen]
pub fn format_text(text: &str, tag: &str) -> String {
    format!("<{}>{}</{}>", tag, text, tag)
}



// Expose function to JavaScript
#[wasm_bindgen]
pub fn format_markdown(input: &str) -> String {
    let mut options = Options::empty();
    let parser = Parser::new_ext(input, options);
    
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}
