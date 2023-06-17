use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct LzFormat;

impl Detector for LzFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_lz(data))
    }
}
impl Validator for LzFormat {
}
