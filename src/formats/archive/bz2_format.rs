use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct Bz2Format;

impl Detector for Bz2Format {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_bz2(data))
    }
}

impl Validator for Bz2Format {

}
