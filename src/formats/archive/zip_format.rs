use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use std::process::Command;
use infer;

pub struct ZipFormat;

impl Detector for ZipFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(true)
    }
}
impl Validator for ZipFormat {
    fn validate(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        let output = Command::new("7za")
            .arg("t")
            .arg(file_path)
            .output()?;

        Ok(output.status.success())
    }
}
