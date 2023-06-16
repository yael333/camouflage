use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct WoffFormat;

impl Detector for WoffFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::font::is_woff(data))
    }
}
impl Validator for WoffFormat {
}
