use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct PsdFormat;

impl Detector for PsdFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_psd(data))
    }
}
impl Validator for PsdFormat {
}
