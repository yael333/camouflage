use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct RarFormat;

impl Detector for RarFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_rar(data))
    }
}
impl Validator for RarFormat {
}
