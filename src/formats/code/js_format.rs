use std::error::Error;
use std::process::Command;


use crate::formats::Detector;
use crate::formats::Validator;

pub struct JsFormat;

// TODO node dependency check?
impl Detector for JsFormat {
    fn detect(&self, _data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(true)
    }
}

impl Validator for JsFormat {
    fn validate(&self, _data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        let output = Command::new("node")
            .arg("-c")
            .arg(file_path)
            .output()?;

        Ok(output.status.success())
    }
}