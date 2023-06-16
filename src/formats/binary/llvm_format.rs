use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct LlvmFormat;

impl Detector for LlvmFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_llvm(data))
    }
}
impl Validator for LlvmFormat {
}
