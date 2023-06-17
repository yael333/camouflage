use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct MsiFormat;

impl Detector for MsiFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_msi(data))
    }
}
impl Validator for MsiFormat {
}
