use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct M4vFormat;

impl Detector for M4vFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::video::is_m4v(data))
    }
}
impl Validator for M4vFormat {
}
