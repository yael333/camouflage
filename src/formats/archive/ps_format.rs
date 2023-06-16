use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct PsFormat;

impl Detector for PsFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_ps(data))
    }
}
impl Validator for PsFormat {
}
