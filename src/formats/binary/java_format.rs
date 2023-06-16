use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct JavaFormat;

impl Detector for JavaFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_java(data))
    }
}
impl Validator for JavaFormat {
}
