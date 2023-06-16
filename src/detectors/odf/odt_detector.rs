use crate::Detector;
use std::error::Error;
use infer;

pub struct OdtDetector;

impl Detector for OdtDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::odf::is_odt(data))
    }
}
