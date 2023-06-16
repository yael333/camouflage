use crate::Detector;
use std::error::Error;
use infer;

pub struct Coff_i386Detector;

impl Detector for Coff_i386Detector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_coff_i386(data))
    }
}
