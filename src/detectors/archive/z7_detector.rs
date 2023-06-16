use crate::Detector;
use std::error::Error;
use infer;

pub struct z7Detector;

impl Detector for z7Detector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_7z(data))
    }
}
