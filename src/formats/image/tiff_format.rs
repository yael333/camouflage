use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct TiffFormat;

impl Detector for TiffFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_tiff(data))
    }
}
impl Validator for TiffFormat {
}
