use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct XzFormat;

impl Detector for XzFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_xz(data))
    }
}
impl Validator for XzFormat {
}
