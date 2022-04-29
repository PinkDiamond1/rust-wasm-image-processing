use crate::engine::ImageProcessingResult;
use cfg_if::cfg_if;
use engine::image_filters::GradientDirection;
use engine::{image_filters::ColorRgba, ImageParameters, ImageProcess};
use image::Rgba;
use image::{DynamicImage, GrayImage, Pixel};
use log::*;
use std::panic;
use wasm_bindgen::prelude::*;

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
    info!("Rust logs initialized");

    //Panic display in the console
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn filter_params(
    base64_input: String,
    params: Option<ImageParameters>,
) -> Result<ImageProcessingResult, JsError> {
    let image_processing = ImageProcess::new(base64_input)?;
    image_processing
        .compute_parameters(params.unwrap_or(ImageParameters::default()))
        .map_err(|e| JsError::new(e.message()))
}

#[wasm_bindgen]
pub fn filter_sobel(base64_input: String) -> Result<ImageProcessingResult, JsError> {
    ImageProcess::new(base64_input)?
        .compute_filter_sobel()
        .map_err(|e| JsError::new(e.message()))
}

#[wasm_bindgen]
pub fn filter_column_color(base64_input: String) -> Result<ImageProcessingResult, JsError> {
    ImageProcess::new(base64_input)?
        .compute_filter_column_color(vec![
            ColorRgba::new(214, 110, 250, 150),
            ColorRgba::new(155, 100, 220, 150),
            ColorRgba::new(150, 120, 240, 150),
            ColorRgba::new(95, 105, 220, 150),
            ColorRgba::new(110, 150, 250, 160),
        ])
        .map_err(|e| JsError::new(e.message()))

    // let image_processing = ImageProcess::new(base64_input, None).unwrap();

    // let mut img = image::load_from_memory(image_processing.input.as_slice()).unwrap();

    // let colors = vec![
    //     ColorRgba::new(214, 110, 250, 150),
    //     ColorRgba::new(155, 100, 220, 150),
    //     ColorRgba::new(150, 120, 240, 150),
    //     ColorRgba::new(95, 105, 220, 150),
    //     ColorRgba::new(110, 150, 250, 160),
    // ];

    // image_filters::filter_col_color(
    //     &mut img,
    //     colors.iter().map(|c| Rgba::<u8>::from(*c)).collect(),
    // )
    // .unwrap();

    // Ok(ImageProcessingResult::new(
    //     ImageProcess::dynamic_image_to_byte(&img),
    // ))
}

#[wasm_bindgen]
pub fn filter_pixel(base64_input: String, pixel_type: engine::image_filters::FilterPixelType) -> Result<ImageProcessingResult, JsError> {
    ImageProcess::new(base64_input)?
        .compute_filter_pixel(pixel_type)
        .map_err(|e| JsError::new(e.message()))
    // let image_processing = ImageProcess::new(base64_input, None).unwrap();

    // let mut img = image::load_from_memory(image_processing.input.as_slice()).unwrap();

    // image_filters::filter_diag(&mut img);

    // Ok(ImageProcessingResult::new(
    //     ImageProcess::dynamic_image_to_byte(&img),
    // ))
}

#[wasm_bindgen]
pub fn filter_gradient(base64_input: String, direction: GradientDirection) -> Result<ImageProcessingResult, JsError> {
    ImageProcess::new(base64_input)?
        .compute_filter_gradient(
            ColorRgba::new(0, 128, 0, 0),
            ColorRgba::new(255, 255, 255, 255),
            direction,
        )
        .map_err(|e| JsError::new(e.message()))

    // let image_processing = ImageProcess::new(base64_input, None).unwrap();

    // let mut img = image::load_from_memory(image_processing.input.as_slice()).unwrap();
    // image_filters::filter_gradient(
    //     &mut img,
    //     ColorRgba::new(0, 128, 0, 0).into(),
    //     ColorRgba::new(255, 255, 255, 255).into(),
    //     GradientDirection::VERTICAL,
    // );

    // Ok(ImageProcessingResult::new(
    //     ImageProcess::dynamic_image_to_byte(&img),
    // ))
}
