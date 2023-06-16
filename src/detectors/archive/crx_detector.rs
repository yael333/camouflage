use crate::Detector;
use std::error::Error;
use infer;

pub struct CrxDetector;

impl Detector for CrxDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_crx(data))
    }
}
