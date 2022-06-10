use crate::engine::{ImageProcessingResult, ImageDimension};
use cfg_if::cfg_if;
use engine::image_filters::GradientDirection;
use engine::{image_filters::ColorRgba, ImageParameters, ImageProcess};
use log::*;
use std::panic;
use wasm_bindgen::prelude::*;

pub mod engine;

// https://github.com/umgefahren/image-comp-lib-rust

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

/// Perform basic filter like Brighteness, huerotate, contrast etc
#[wasm_bindgen]
pub fn image_resize(
    base64_input: String,
    width: usize,
    height: usize,
) -> Result<ImageProcessingResult, JsError> {
    let image_processing = ImageProcess::new(base64_input)?;
    image_processing
        .resize(width, height)
        .map_err(|e| JsError::new(e.message()))
}

/// Perform basic filter like Brighteness, huerotate, contrast etc
#[wasm_bindgen]
pub fn image_weight(base64_input: String) -> Result<usize, JsError> {
    ImageProcess::get_image_weight_byte(base64_input).map_err(|e| JsError::new(e.message()))
}

#[wasm_bindgen]
pub fn image_dimension(base64_input: String) -> Result<ImageDimension, JsError> {
    ImageProcess::new(base64_input)?.get_image_dimension().map_err(|e| JsError::new(e.message()))
}

#[wasm_bindgen]
pub fn calc_best_size_ratio(base64_input: String, target_size: usize) -> Result<ImageProcessingResult, JsError> {
    ImageProcess::calc_best_size_ratio(base64_input, target_size).map_err(|e| JsError::new(e.message()))
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

/// Perform a Sobel filter
#[wasm_bindgen]
pub fn filter_sobel(base64_input: String) -> Result<ImageProcessingResult, JsError> {
    ImageProcess::new(base64_input)?
        .compute_filter_sobel()
        .map_err(|e| JsError::new(e.message()))
}

/// Perform a filter with colored band (vertical or horizontal)
#[wasm_bindgen]
pub fn filter_overlay_color(
    base64_input: String,
    direction: GradientDirection,
) -> Result<ImageProcessingResult, JsError> {
    ImageProcess::new(base64_input)?
        .compute_filter_band_color(
            vec![
                ColorRgba::new(214, 110, 250, 150),
                ColorRgba::new(155, 100, 220, 150),
                ColorRgba::new(150, 120, 240, 150),
                ColorRgba::new(95, 105, 220, 150),
                ColorRgba::new(110, 150, 250, 160),
            ],
            direction,
        )
        .map_err(|e| JsError::new(e.message()))
}

/// Perform basic filter with lines of pixel drawn on the top of the image
#[wasm_bindgen]
pub fn filter_pixel(
    base64_input: String,
    pixel_type: engine::image_filters::FilterPixelType,
    color: ColorRgba,
) -> Result<ImageProcessingResult, JsError> {
    ImageProcess::new(base64_input)?
        .compute_filter_pixel(pixel_type, color)
        .map_err(|e| JsError::new(e.message()))
}

/// Perform a vertical or horizontal linear gradient
#[wasm_bindgen]
pub fn filter_gradient(
    base64_input: String,
    direction: GradientDirection,
    from: ColorRgba,
    to: ColorRgba,
) -> Result<ImageProcessingResult, JsError> {
    ImageProcess::new(base64_input)?
        .compute_filter_gradient(from, to, direction)
        .map_err(|e| JsError::new(e.message()))
}
