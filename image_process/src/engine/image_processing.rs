use std::io::Cursor;
use chrono::Local;
use crate::ErrorCode;
use image::{ImageResult, DynamicImage};
use wasm_bindgen::prelude::*;
use log::*;


#[derive(Debug)]
pub struct ImageProcess {
    pub base64_input: String,
    pub base64_output: Option<String>,
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

        if let Some(_) = self.grayscale {
            img = img.grayscale();
            info!("Grayscale filter applied");
        }

        if let Some(constrast) = self.constrast {
            img = img.adjust_contrast(constrast);
            info!("Constrast filter applied : {}", constrast);
        }

        img
    }
}

impl ImageProcess {
    pub fn new(base64_input: String, filter: Option<ImageParameters>) -> ImageProcess {
        info!("New ImageProcess instance");
        ImageProcess {
            base64_input: ImageProcess::parse_base64_input_if_needed(base64_input),
            base64_output: None,
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
        }
    }

    pub fn parse_base64_input_if_needed(base64_input : String) -> String {
        str::replace(base64_input.as_str(), "data:image/jpeg;base64,", "")
    }

    // pub fn base64_input(&self) -> &String {
    //     &self.base64_input
    // }

    // pub fn base64_output(&self) -> Option<&String> {
    //     self.base64_output.as_ref()
    // }

    pub fn update_output(&mut self, output : &str) {
        self.base64_output = Some(String::from(output));
    }

    pub fn image_output(&self) -> Option<ImageResult<DynamicImage>> {
        match &self.base64_output {
            Some(base64_output_ok) => {
                let img_bytes = base64::decode(base64_output_ok).unwrap();
                let image_loaded_result = image::load_from_memory(&img_bytes.as_slice());

                Some(image_loaded_result)
            },
            None => None
        }
    }

    pub fn compute_image_processing(&self) -> Result<String, ErrorCode> {
        //Decode the base64 image and get list of byte
        info!("compute_image_processing()");
        let img_bytes = match base64::decode(&self.base64_input) {
            Ok(img_bytes) => {
                info!("Convert Vec<u8> byte from base64 image");
                img_bytes
            },
            Err(e) => {
                error!("Failed base64::decode() : {}", e);
                return Err(ErrorCode::InvalidParsing)
            }
        };

        //Load image from byte
        info!("Try to create Dynamic image from byte");
        let mut img = image::load_from_memory(&img_bytes.as_slice()).unwrap();
        info!("Dynamic image instance created");

        img = self.filter_params.apply_filter(img.clone());

        info!("Convert filtered image to bytes");
        let mut edited_image_bytes = Vec::new();
        img.write_to(&mut Cursor::new(&mut edited_image_bytes), image::ImageOutputFormat::Png).unwrap();

        info!("Encode bytes filtered image to base64");
        Ok(base64::encode(&edited_image_bytes))
    }

    pub fn compute_image_processing_buf(&self) -> Result<Vec<u8>, ErrorCode> {
        //Decode the base64 image and get list of byte
        info!("compute_image_processing()");
        let img_bytes = match base64::decode(&self.base64_input) {
            Ok(img_bytes) => {
                info!("Convert Vec<u8> byte from base64 image");
                img_bytes
            },
            Err(e) => {
                error!("Failed base64::decode() : {}", e);
                return Err(ErrorCode::InvalidParsing)
            }
        };

        //Load image from byte
        info!("Try to create Dynamic image from byte");
        let mut img = image::load_from_memory(&img_bytes.as_slice()).unwrap();
        info!("Dynamic image instance created");

        img = self.filter_params.apply_filter(img.clone());

        info!("Convert filtered image to bytes");
        let mut edited_image_bytes = Vec::new();
        img.write_to(&mut Cursor::new(&mut edited_image_bytes), image::ImageOutputFormat::Png).unwrap();

        Ok(edited_image_bytes)
        // info!("Encode bytes filtered image to base64");
        // Ok(base64::encode(&edited_image_bytes))
    }

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