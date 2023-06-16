use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct PngFormat;

impl Detector for PngFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_png(data))
    }
}
impl Validator for PngFormat {
}
