use crate::Detector;
use std::error::Error;
use infer;

pub struct OtfDetector;

impl Detector for OtfDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::font::is_otf(data))
    }
}
