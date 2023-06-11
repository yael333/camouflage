use crate::Detector;
use std::error::Error;
use infer;

pub struct PdfDetector;

impl Detector for PdfDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(data.windows(5).any(|window| window == b"%PDF-")) // Firefox Version 
    }
}