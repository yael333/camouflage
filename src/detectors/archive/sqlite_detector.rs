use crate::Detector;
use std::error::Error;
use infer;

pub struct SqliteDetector;

impl Detector for SqliteDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_sqlite(data))
    }
}
