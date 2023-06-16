use crate::Detector;
use std::error::Error;
use infer;

pub struct DerDetector;

impl Detector for DerDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_der(data))
    }
}
