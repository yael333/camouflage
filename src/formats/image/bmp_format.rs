use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct BmpFormat;

impl Detector for BmpFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_bmp(data))
    }
}
impl Validator for BmpFormat {
}
