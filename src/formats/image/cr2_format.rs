use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct Cr2Format;

impl Detector for Cr2Format {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_cr2(data))
    }
}
impl Validator for Cr2Format {
}
