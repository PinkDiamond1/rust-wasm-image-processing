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