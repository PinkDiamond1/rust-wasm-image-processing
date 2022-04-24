use std::{io::Cursor, fmt::Display};
use chrono::Local;
// use crate::ErrorCode;
use image::{ImageResult, DynamicImage};
use wasm_bindgen::prelude::*;
use log::*;
use super::{ErrorCode, ImageProcessingResult};

pub trait InputType {
    fn to_byte(&self) -> Result<Vec<u8>, ErrorCode>;
}
impl InputType for String {
    fn to_byte(&self) -> Result<Vec<u8>, ErrorCode> {
        match base64::decode(ImageProcess::parse_base64_input_if_needed(&self)) {
            Ok(img_bytes) => {
                info!("Convert Vec<u8> byte from base64 image");
                return Ok(img_bytes);
            },
            Err(e) => {
                error!("Failed base64::decode() : {}", e);
                return Err(ErrorCode::UnableToDecode)
            }
        }
    }
}
impl InputType for Vec<u8> {
    fn to_byte(&self) -> Result<Vec<u8>, ErrorCode> {
        Ok(self.to_vec())
    }
}

#[derive(Debug)]
pub struct ImageProcess
{
    pub input: Vec<u8>,
    pub filter_params: ImageParameters
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct ImageParameters {
    pub brighten: Option<i32>,
    pub blur: Option<f32>,
    pub hue: Option<i32>,
    pub grayscale: Option<bool>,
    pub constrast: Option<f32>
}

#[wasm_bindgen]
impl ImageParameters {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ImageParameters {
        ImageParameters::default()
    }
}

impl ImageParameters {
    pub fn default() -> Self {
        Self {
            brighten : Some(0),
            blur : Some(0.0),
            hue : Some(0),
            grayscale : Some(false),
            constrast: Some(0.0)
        }
    }

    pub fn apply_filter(&self, mut img: DynamicImage) -> DynamicImage {
        if let Some(brighten) = self.brighten {
            img = img.brighten(brighten);
            info!("Brighten filter applied : {}", brighten);
        }

        if let Some(blur) = self.blur {
            img = img.blur(blur);
            info!("Blur filter applied : {}", blur);
        }

        if let Some(hue) = self.hue {
            img = img.huerotate(hue);
            info!("Huerotate filter applied : {}", hue);
        }

        if let Some(b) = self.grayscale {
            if b {
                img = img.grayscale();
                info!("Grayscale filter applied");
            }
        }

        if let Some(constrast) = self.constrast {
            img = img.adjust_contrast(constrast);
            info!("Constrast filter applied : {}", constrast);
        }

        img
    }
}

impl ImageProcess {
    pub fn new<T>(input: T, filter: Option<ImageParameters>) -> Result<ImageProcess, ErrorCode>
        where T : InputType
    {
        info!("New ImageProcess instance");
        Ok(ImageProcess {
            input: input.to_byte()?,
            filter_params : match filter {
                Some(filter) => {
                    info!("Filters parameter set");
                    filter
                },
                None => {
                    info!("Not filter set, use default one");
                    ImageParameters::default()
                }
            }
        })
    }

    //Replace the "data:image/jpeg;base64," in the string
    pub fn parse_base64_input_if_needed(base64_input : &String) -> String {
        str::replace(base64_input.as_str(), "data:image/jpeg;base64,", "")
    }

    //Create a Dynamic image from base64
    pub fn get_image(base64: &str) -> ImageResult<DynamicImage> {
        let img_bytes = base64::decode(base64).unwrap();
        image::load_from_memory(&img_bytes.as_slice())
    }

    pub fn compute_image_processing(&self) -> Result<ImageProcessingResult, ErrorCode> {
        //Load image from byte
        info!("Try to create Dynamic image from byte");
        let mut img = image::load_from_memory(&self.input.as_slice()).unwrap();
        info!("Dynamic image instance created");

        img = self.filter_params.apply_filter(img.clone());

        info!("Convert filtered image to bytes");
        let mut edited_image_bytes = Vec::new();
        img.write_to(&mut Cursor::new(&mut edited_image_bytes), image::ImageOutputFormat::Png).unwrap();

        Ok(ImageProcessingResult::new(edited_image_bytes))
    }

    // pub fn compute_image_processing_as_base64(&self) -> Result<String, ErrorCode> {
    //     let filter_image_bytes = self.compute_image_processing_as_byte()?;

    //     info!("Encode bytes filtered image to base64");
    //     Ok(base64::encode(&filter_image_bytes))
    // }

    pub fn save_image(img: &DynamicImage, path: &str) -> Option<ErrorCode> {
        let local_date = Local::now();
        let date_string = local_date.format("%Y-%m-%d_%H-%M-%S").to_string();
        let full_file_path = format!("{}image_save_{}.png", &path, &date_string);

        println!("Full path to save : {}", &full_file_path);

        if let Err(_) = img.save(full_file_path) {
            return Some(ErrorCode::UnableToSave);
        }
        None
    }
}

impl Display for ImageProcess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", base64::encode(&self.input))
    }
}