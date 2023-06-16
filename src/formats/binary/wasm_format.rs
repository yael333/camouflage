use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct WasmFormat;

impl Detector for WasmFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_wasm(data))
    }
}
impl Validator for WasmFormat {
}
