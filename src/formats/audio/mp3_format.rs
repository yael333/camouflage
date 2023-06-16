use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct Mp3Format;

impl Detector for Mp3Format {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_mp3(data))
    }
}
impl Validator for Mp3Format {
}
