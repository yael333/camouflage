use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct FlacFormat;

impl Detector for FlacFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_flac(data))
    }
}
impl Validator for FlacFormat {
}
