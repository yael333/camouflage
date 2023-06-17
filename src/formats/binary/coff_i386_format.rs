use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct Coff_i386Format;

impl Detector for Coff_i386Format {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_coff_i386(data))
    }
}
impl Validator for Coff_i386Format {
}
