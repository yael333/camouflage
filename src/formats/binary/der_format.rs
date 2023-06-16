use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct DerFormat;

impl Detector for DerFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_der(data))
    }
}
impl Validator for DerFormat {
}
