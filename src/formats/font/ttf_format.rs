use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct TtfFormat;

impl Detector for TtfFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::font::is_ttf(data))
    }
}
impl Validator for TtfFormat {
}
