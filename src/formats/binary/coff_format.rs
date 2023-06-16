use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct CoffFormat;

impl Detector for CoffFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_coff(data))
    }
}
impl Validator for CoffFormat {
}
