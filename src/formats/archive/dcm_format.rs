use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct DcmFormat;

impl Detector for DcmFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_dcm(data))
    }
}
impl Validator for DcmFormat {
}
