use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct DexFormat;

impl Detector for DexFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_dex(data))
    }
}
impl Validator for DexFormat {
}
