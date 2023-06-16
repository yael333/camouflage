use crate::Detector;
use std::error::Error;
use infer;

pub struct FlacDetector;

impl Detector for FlacDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_flac(data))
    }
}
