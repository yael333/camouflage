use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct AmrFormat;

impl Detector for AmrFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_amr(data))
    }
}
impl Validator for AmrFormat {
}
