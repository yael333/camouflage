use crate::Detector;
use std::error::Error;
use infer;

pub struct Coff_ia64Detector;

impl Detector for Coff_ia64Detector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_coff_ia64(data))
    }
}
