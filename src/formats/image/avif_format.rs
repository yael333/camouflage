use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct AvifFormat;

impl Detector for AvifFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_avif(data))
    }
}
impl Validator for AvifFormat {
}
