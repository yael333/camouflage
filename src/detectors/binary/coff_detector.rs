use crate::Detector;
use std::error::Error;
use infer;

pub struct CoffDetector;

impl Detector for CoffDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_coff(data))
    }
}
