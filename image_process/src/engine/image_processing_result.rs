use wasm_bindgen::prelude::*;

/// Result structure after image processing has been apply
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct ImageProcessingResult {
    result: Vec<u8>
}

#[wasm_bindgen]
impl ImageProcessingResult {
    pub fn to_byte(&self) -> Vec<u8> {
        self.result.clone()
    }

    pub fn to_base64(&self) -> String {
        base64::encode(&self.result)
    }
}

impl ImageProcessingResult {
    pub fn new(result: Vec<u8>) -> ImageProcessingResult {
        ImageProcessingResult { result }
    }
}

#[wasm_bindgen]
pub struct ImageDimension {
    width: usize,
    height: usize
}

#[wasm_bindgen]
impl ImageDimension {
    pub fn new(width: u32, height: u32) -> ImageDimension {
        ImageDimension { width: width as usize, height: height as usize }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}