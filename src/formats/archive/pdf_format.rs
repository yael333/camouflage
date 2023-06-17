use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;

pub struct PdfFormat;

impl Detector for PdfFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(data[..usize::min(data.len(), 1024)].windows(5).any(|window| window == b"%PDF-"))
    }
}impl Validator for PdfFormat {
}
