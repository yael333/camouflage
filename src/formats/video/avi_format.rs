use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct AviFormat;

impl Detector for AviFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::video::is_avi(data))
    }
}
impl Validator for AviFormat {
}
