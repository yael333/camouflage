use crate::Detector;
use std::error::Error;
use infer;

pub struct M4aDetector;

impl Detector for M4aDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_m4a(data))
    }
}
