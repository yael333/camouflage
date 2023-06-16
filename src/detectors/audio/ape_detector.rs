use crate::Detector;
use std::error::Error;
use infer;

pub struct ApeDetector;

impl Detector for ApeDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_ape(data))
    }
}
