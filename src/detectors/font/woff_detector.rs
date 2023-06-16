use crate::Detector;
use std::error::Error;
use infer;

pub struct WoffDetector;

impl Detector for WoffDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::font::is_woff(data))
    }
}
