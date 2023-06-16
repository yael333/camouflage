use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct MachFormat;

impl Detector for MachFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_mach(data))
    }
}
impl Validator for MachFormat {
}
