use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct IcoFormat;

impl Detector for IcoFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_ico(data))
    }
}
impl Validator for IcoFormat {
}
