use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct HtmlFormat;

impl Detector for HtmlFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(data[..usize::min(data.len(), 1024)].windows(5).any(|window| window == b"%PDF-" || window == b"<html" || window == b"body" || window == b"<script"))
    }
}

impl Validator for HtmlFormat {
}