use crate::Detector;
use std::error::Error;
use infer;

pub struct WavDetector;

impl Detector for WavDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_wav(data))
    }
}
