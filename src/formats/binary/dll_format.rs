use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct DllFormat;

impl Detector for DllFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_dll(data))
    }
}
impl Validator for DllFormat {
}
