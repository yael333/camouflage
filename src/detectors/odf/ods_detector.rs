use crate::Detector;
use std::error::Error;
use infer;

pub struct OdsDetector;

impl Detector for OdsDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::odf::is_ods(data))
    }
}
