use crate::Detector;
use std::error::Error;
use infer;

pub struct M4aDetector;

impl Detector for M4aDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_m4a(data))
    }
}
