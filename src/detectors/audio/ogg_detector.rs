use crate::Detector;
use std::error::Error;
use infer;

pub struct OggDetector;

impl Detector for OggDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_ogg(data))
    }
}
