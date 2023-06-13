use crate::Detector;
use std::error::Error;
use infer;

pub struct WebmDetector;

impl Detector for WebmDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::video::is_webm(data))
    }
}
