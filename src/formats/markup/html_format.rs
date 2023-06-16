use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct HtmlFormat;

impl Detector for HtmlFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::text::is_html(data))
    }
}
impl Validator for HtmlFormat {
}
