use crate::Detector;
use std::error::Error;
use infer;

pub struct LlvmDetector;

impl Detector for LlvmDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_llvm(data))
    }
}
