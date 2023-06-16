use crate::Detector;
use std::error::Error;
use infer;

pub struct TtfDetector;

impl Detector for TtfDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::font::is_ttf(data))
    }
}
