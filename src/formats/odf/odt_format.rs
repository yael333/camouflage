use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct OdtFormat;

impl Detector for OdtFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::odf::is_odt(data))
    }
}

impl Validator for OdtFormat {
}
