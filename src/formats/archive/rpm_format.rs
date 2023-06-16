use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct RpmFormat;

impl Detector for RpmFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_rpm(data))
    }
}
impl Validator for RpmFormat {
}
