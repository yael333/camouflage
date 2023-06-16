use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct RtfFormat;

impl Detector for RtfFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_rtf(data))
    }
}
impl Validator for RtfFormat {
}
