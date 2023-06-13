use crate::Detector;
use std::error::Error;
use infer;

pub struct DocxDetector;

impl Detector for DocxDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::doc::is_docx(data))
    }
}
