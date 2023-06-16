use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct OggFormat;

impl Detector for OggFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_ogg(data))
    }
}
impl Validator for OggFormat {
}
