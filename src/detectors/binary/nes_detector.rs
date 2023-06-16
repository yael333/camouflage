use crate::Detector;
use std::error::Error;
use infer;

pub struct NesDetector;

impl Detector for NesDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_nes(data))
    }
}
