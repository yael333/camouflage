use crate::Detector;
use std::error::Error;

pub struct JpegDetector;

impl Detector for JpegDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(data.starts_with(&[0xFF, 0xD8, 0xFF]))
    }
}
