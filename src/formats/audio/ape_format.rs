use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct ApeFormat;

impl Detector for ApeFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_ape(data))
    }
}
impl Validator for ApeFormat {
}
