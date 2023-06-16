use crate::Detector;
use std::error::Error;
use infer;

pub struct OdpDetector;

impl Detector for OdpDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::odf::is_odp(data))
    }
}
