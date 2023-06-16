use crate::Detector;
use std::error::Error;
use infer;

pub struct JavaDetector;

impl Detector for JavaDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_java(data))
    }
}
