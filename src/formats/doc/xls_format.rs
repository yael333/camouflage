use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct XlsFormat;

impl Detector for XlsFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::doc::is_xls(data))
    }
}
impl Validator for XlsFormat {
}
