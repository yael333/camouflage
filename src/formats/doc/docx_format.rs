use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct DocxFormat;

impl Detector for DocxFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::doc::is_docx(data))
    }
}
impl Validator for DocxFormat {
}
