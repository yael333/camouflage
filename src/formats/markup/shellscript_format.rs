use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct ShellscriptFormat;

impl Detector for ShellscriptFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::text::is_shellscript(data))
    }
}
impl Validator for ShellscriptFormat {
}
