use crate::Detector;
use std::error::Error;
use infer;

pub struct XlsxDetector;

impl Detector for XlsxDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::doc::is_xlsx(data))
    }
}
