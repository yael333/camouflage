use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct PemFormat;

impl Detector for PemFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_pem(data))
    }
}
impl Validator for PemFormat {
}
