use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct EpubFormat;

impl Detector for EpubFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_epub(data))
    }
}
impl Validator for EpubFormat {
}