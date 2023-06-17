use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct XmlFormat;

impl Detector for XmlFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::text::is_xml(data))
    }
}
impl Validator for XmlFormat {
}
