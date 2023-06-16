use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct CrxFormat;

impl Detector for CrxFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_crx(data))
    }
}
impl Validator for CrxFormat {
}
