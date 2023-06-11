use std::error::Error;
use std::process::Command;
use std::io::Write;
use std::fs::File;
use crate::detectors::Detector;
use std::ascii::escape_default;

pub struct JsDetector;

// TODO node dependency check?
impl Detector for JsDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        let temp_file_path = "/tmp/temp.js"; // TODO windows support? 
        let mut temp_file = File::create(temp_file_path)?;
        temp_file.write_all(data)?;

        let output = Command::new("node")
            .arg("-c")
            .arg(temp_file_path)
            .output()?;

        std::fs::remove_file(temp_file_path)?;

        Ok(output.status.success())
    }
}
