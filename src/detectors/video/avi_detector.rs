use crate::Detector;
use std::error::Error;
use infer;

pub struct AviDetector;

impl Detector for AviDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::video::is_avi(data))
    }
}
