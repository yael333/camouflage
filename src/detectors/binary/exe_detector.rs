use crate::Detector;
use std::error::Error;
use infer;

pub struct ExeDetector;

impl Detector for ExeDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_exe(data))
    }
}
