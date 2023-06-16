use crate::Detector;
use std::error::Error;
use infer;

pub struct WoffDetector;

impl Detector for WoffDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::font::is_woff(data))
    }
}
