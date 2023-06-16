use crate::Detector;
use std::error::Error;
use infer;

pub struct Coff_x64Detector;

impl Detector for Coff_x64Detector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_coff_x64(data))
    }
}
