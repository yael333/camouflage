use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct PptxFormat;

impl Detector for PptxFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::doc::is_pptx(data))
    }
}
impl Validator for PptxFormat {
}
