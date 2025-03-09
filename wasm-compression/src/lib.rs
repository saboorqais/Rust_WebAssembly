use wasm_bindgen::prelude::*;
use image::{DynamicImage, io::Reader as ImageReader};
use image::codecs::jpeg::JpegEncoder;
use std::io::Cursor;

#[wasm_bindgen]
pub fn compress_image(input: &[u8], quality: u8) -> Vec<u8> {
    let img = ImageReader::new(Cursor::new(input))
        .with_guessed_format()
        .expect("Failed to read image")
        .decode()
        .expect("Failed to decode image");

    let mut compressed_data = Vec::new();
    let mut encoder = JpegEncoder::new_with_quality(&mut compressed_data, quality);
    
    encoder.encode_image(&img).expect("Failed to compress image");

    compressed_data
}
