use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct Coff_ia64Format;

impl Detector for Coff_ia64Format {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_coff_ia64(data))
    }
}
impl Validator for Coff_ia64Format {
}
