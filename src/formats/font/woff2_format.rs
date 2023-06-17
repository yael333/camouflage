use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct Woff2Format;

impl Detector for Woff2Format {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::font::is_woff2(data))
    }
}
impl Validator for Woff2Format {
}
