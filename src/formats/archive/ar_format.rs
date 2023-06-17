use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct ArFormat;

impl Detector for ArFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_ar(data))
    }
}

impl Validator for ArFormat {

}
