use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct Mp4Format;

impl Detector for Mp4Format {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::video::is_mp4(data))
    }
}
impl Validator for Mp4Format {
}
