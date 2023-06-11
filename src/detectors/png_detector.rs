use crate::Detector;
use std::error::Error;

pub struct PngDetector;

impl Detector for PngDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(data.starts_with(&[137, 80, 78, 71, 13, 10, 26, 10]))
    }
}