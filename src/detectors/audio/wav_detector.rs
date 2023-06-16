use crate::Detector;
use std::error::Error;
use infer;

pub struct WavDetector;

impl Detector for WavDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_wav(data))
    }
}
