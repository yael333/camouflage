use crate::Detector;
use std::error::Error;
use infer;

pub struct MachDetector;

impl Detector for MachDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_mach(data))
    }
}