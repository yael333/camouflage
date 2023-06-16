use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct AiffFormat;

impl Detector for AiffFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_aiff(data))
    }
}
impl Validator for AiffFormat {
}
