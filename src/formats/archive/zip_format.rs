use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct ZipFormat;

impl Detector for ZipFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_zip(data))
    }
}
impl Validator for ZipFormat {
}
