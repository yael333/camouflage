use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct EotFormat;

impl Detector for EotFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_eot(data))
    }
}
impl Validator for EotFormat {
}
