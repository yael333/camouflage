use crate::Detector;
use std::error::Error;
use infer;

pub struct CpioDetector;

impl Detector for CpioDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_cpio(data))
    }
}
