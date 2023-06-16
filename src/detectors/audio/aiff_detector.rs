use crate::Detector;
use std::error::Error;
use infer;

pub struct AiffDetector;

impl Detector for AiffDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_aiff(data))
    }
}
