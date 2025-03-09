use wasm_bindgen::prelude::*;
use pulldown_cmark::{Parser, Options, html};
// Expose functions to JavaScript

const KEYWORDS: [&str; 5] = ["fn", "let", "if", "else", "return"];

#[wasm_bindgen]
pub fn highlight_code(input :&str)->String{
    let mut output = String::new();
   
    for word in input.split_whitespace(){
        if KEYWORDS.contains(&word){
            output.push_str(&format!("<span style='color: blue;'>{}</span> ", word));
        }else{
            output.push_str(&format!("{} ", word));
        }
        output.push(' ');
    }
    output.trim().to_string()
}

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
