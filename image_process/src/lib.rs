use std::panic;

use engine::{ImageProcess, ImageParameters, ErrorCode};
use wasm_bindgen::prelude::*;
use log::*;
use cfg_if::cfg_if;

pub mod engine;

cfg_if! {
    if #[cfg(feature = "console_log")] {
        fn init_log() {
            use log::Level;
            console_log::init_with_level(Level::Trace).expect("error initializing log");
        }
    } else {
        fn init_log() {}
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    init_log();
    info!("Console log initialized");

    //Panic display in the console
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn perform_processing(base64_input: String, filter: Option<ImageParameters>) -> Vec<u8> {
// pub fn perform_processing(base64_input: String, filter: Option<ImageParameters>) -> Result<String, JsError> {
// pub fn perform_processing(base64_input: String) -> String {
    info!("Call perform processing");
    let image_processing = ImageProcess::new(base64_input, filter);
    info!("Input : {}", image_processing.base64_input);
    //info!("ImageProcessing instance created : {:?}", image_processing);

    image_processing.compute_image_processing_buf().unwrap()
    // match image_processing.compute_image_processing() {
    //     Ok(res) => Ok(res),
    //     Err(e) => Err(JsError::new(e.message()))
    // }
}

#[wasm_bindgen]
pub fn test(base64_img: String) {
    info!("base64_img = {}", &base64_img[0..40]);
    let parsed_base64_img = ImageProcess::parse_base64_input_if_needed(base64_img);
    info!("parsed_base64_img = {}", &parsed_base64_img[0..40]);
    let img_bytes = base64::decode(&parsed_base64_img).unwrap();
    info!("img_bytes = {:?}", img_bytes);
    let img = image::load_from_memory(&img_bytes.as_slice()).unwrap();
    info!("ok dynamic image = {:?}", img);
}