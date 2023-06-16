use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct OdpFormat;

impl Detector for OdpFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::odf::is_odp(data))
    }
}
impl Validator for OdpFormat {
}