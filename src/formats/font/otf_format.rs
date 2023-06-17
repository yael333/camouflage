use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct OtfFormat;

impl Detector for OtfFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::font::is_otf(data))
    }
}
impl Validator for OtfFormat {
}
