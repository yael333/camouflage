use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct AacFormat;

impl Detector for AacFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_aac(data))
    }
}
impl Validator for AacFormat {
}
