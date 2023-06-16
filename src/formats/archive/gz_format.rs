use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct GzFormat;

impl Detector for GzFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_gz(data))
    }
}
impl Validator for GzFormat {
}
