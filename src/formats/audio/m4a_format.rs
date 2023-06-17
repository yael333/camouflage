use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct M4aFormat;

impl Detector for M4aFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_m4a(data))
    }
}
impl Validator for M4aFormat {
}
