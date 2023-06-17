use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct TarFormat;

impl Detector for TarFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_tar(data))
    }
}
impl Validator for TarFormat {
}
