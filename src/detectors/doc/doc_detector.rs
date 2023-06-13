use crate::Detector;
use std::error::Error;
use infer;

pub struct DocDetector;

impl Detector for DocDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::doc::is_doc(data))
    }
}
