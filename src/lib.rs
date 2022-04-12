use base64::{decode, encode};
use image::ImageOutputFormat::Png;
use image::{buffer, load_from_memory, DynamicImage, ImageResult};
use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;

fn decode_file(encoded_file: &str, fn_name: &str) -> DynamicImage {
    log(&fn_name.into());
    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    let img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());

    return img;
}

#[wasm_bindgen]
pub fn greyscale(encoded_file: &str) -> String {
    let mut img = decode_file(encoded_file, "grayscale was called");

    img = img.grayscale();
    log(&"grayscale effect applied".into());

    let mut buffer = vec![];
    img.write_to(&mut buffer, Png).unwrap();
    log(&"new image written".into());

    let encoded_img = encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", encoded_img);

    return data_url;
}

#[wasm_bindgen]
pub fn blur(encoded_file: &str) -> String {
    let mut img = decode_file(encoded_file, "blur  was called");

    img = img.blur(3.3);
    log(&"blur effect applied".into());

    let mut buffer = vec![];
    img.write_to(&mut buffer, Png).unwrap();
    log(&"new image written".into());

    let encoded_img = encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", encoded_img);

    return data_url;
}
