use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct WavFormat;

impl Detector for WavFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_wav(data))
    }
}
impl Validator for WavFormat {
}
