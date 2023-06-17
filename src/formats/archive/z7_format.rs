use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct Z7Format;

impl Detector for Z7Format {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_7z(data))
    }
}

impl Validator for Z7Format {
}
