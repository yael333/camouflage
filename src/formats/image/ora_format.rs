use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct OraFormat;

impl Detector for OraFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_ora(data))
    }
}
impl Validator for OraFormat {
}
