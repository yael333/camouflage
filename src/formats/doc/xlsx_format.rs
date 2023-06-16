use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct XlsxFormat;

impl Detector for XlsxFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::doc::is_xlsx(data))
    }
}
impl Validator for XlsxFormat {
}
