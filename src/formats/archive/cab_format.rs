use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct CabFormat;

impl Detector for CabFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_cab(data))
    }
}
impl Validator for CabFormat {
}
