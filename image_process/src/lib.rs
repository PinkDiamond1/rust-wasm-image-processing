use crate::engine::{image_filters::FilterType as EngineFilterType, ImageProcessingResult};
use cfg_if::cfg_if;
use engine::image_filters::{self, GradientDirection};
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
pub fn perform_processing(
    base64_input: String,
    filter: Option<ImageParameters>,
) -> Result<ImageProcessingResult, JsError> {
    let image_processing = ImageProcess::new(base64_input, filter)?;

    match image_processing.compute_image_processing() {
        Ok(res) => Ok(res),
        Err(e) => Err(JsError::new(e.message())),
    }
}

#[wasm_bindgen]
pub fn perform_filter(
    base64_input: String,
    filter: EngineFilterType,
) -> Result<ImageProcessingResult, JsError> {
    let image_processing = ImageProcess::new(base64_input, None).unwrap();

    let img = image::load_from_memory(image_processing.input.as_slice()).unwrap();
    let gray_image: GrayImage = img.to_luma8();

    // let contours = imageproc::contours::find_contours(&gray_image);
    let sobel = imageproc::gradients::sobel_gradients(&gray_image);
    // let prewitt = imageproc::gradients::prewitt_gradients(&gray_image);
    // let x: ImageBuffer<Luma<u16>, Vec<u16>> = prewitt.convert();
    let sobel_image = DynamicImage::from(sobel);

    Ok(ImageProcessingResult::new(
        ImageProcess::dynamic_image_to_byte(&sobel_image),
    ))
}

#[wasm_bindgen]
pub fn filter_col_color(base64_input: String) -> Result<ImageProcessingResult, JsError> {
    let image_processing = ImageProcess::new(base64_input, None).unwrap();

    let mut img = image::load_from_memory(image_processing.input.as_slice()).unwrap();

    let colors = vec![
        ColorRgba::new(214, 110, 250, 150),
        ColorRgba::new(155, 100, 220, 150),
        ColorRgba::new(150, 120, 240, 150),
        ColorRgba::new(95, 105, 220, 150),
        ColorRgba::new(110, 150, 250, 160),
    ];

    image_filters::filter_col_color(&mut img, colors.iter().map(|c| Rgba::<u8>::from(*c)).collect()).unwrap();

    Ok(ImageProcessingResult::new(
        ImageProcess::dynamic_image_to_byte(&img),
    ))
}

#[wasm_bindgen]
pub fn filter_vertical(base64_input: String) -> Result<ImageProcessingResult, JsError> {
    let image_processing = ImageProcess::new(base64_input, None).unwrap();

    let mut img = image::load_from_memory(image_processing.input.as_slice()).unwrap();

    image_filters::filter_diag(&mut img);

    Ok(ImageProcessingResult::new(
        ImageProcess::dynamic_image_to_byte(&img),
    ))
}

#[wasm_bindgen]
pub fn filter_gradient(base64_input: String) -> Result<ImageProcessingResult, JsError> {
    let image_processing = ImageProcess::new(base64_input, None).unwrap();

    let mut img = image::load_from_memory(image_processing.input.as_slice()).unwrap();
    
    image_filters::filter_gradient(
        &mut img,
        ColorRgba::new(0, 128, 0, 0).into(),
        ColorRgba::new(255, 255, 255, 255).into(),
        GradientDirection::VERTICAL,
    );

    Ok(ImageProcessingResult::new(
        ImageProcess::dynamic_image_to_byte(&img),
    ))
}