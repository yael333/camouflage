use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct HeifFormat;

impl Detector for HeifFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_heif(data))
    }
}
impl Validator for HeifFormat {
}
