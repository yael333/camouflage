use crate::Detector;
use std::error::Error;

pub struct PdfDetector;

impl Detector for PdfDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(data[..usize::min(data.len(), 1024)].windows(5).any(|window| window == b"%PDF-"))
    }
}