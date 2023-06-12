mod archive;
mod audio;
mod binary;
mod code;
mod doc;
mod font;
mod image;
mod markup;
mod odf;
mod video;

pub use self::image::PngDetector;
pub use self::image::JpegDetector;
pub use self::binary::WasmDetector;
pub use self::binary::ElfDetector;
pub use self::binary::ExeDetector;
pub use self::binary::MachDetector;
pub use self::archive::ZipDetector;
pub use self::archive::PdfDetector;
pub use self::code::JsDetector;
pub use self::video::MovDetector;



use std::error::Error;

pub trait Detector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>>;
}

// Function to return a list of all available detectors
pub fn get_available_detectors() -> Vec<(Box<dyn Detector + 'static>, &'static str)> {
    vec![
        (Box::new(PngDetector), "PNG"),
        (Box::new(JpegDetector), "JPEG"),
        (Box::new(WasmDetector), "WASM"),
        (Box::new(ExeDetector), "EXE"),
        (Box::new(ElfDetector), "ELF"),
        (Box::new(MachDetector), "MACH"),
        (Box::new(ZipDetector), "ZIP"),
        (Box::new(PdfDetector), "PDF"),
        (Box::new(JsDetector), "JS"),
        (Box::new(MovDetector), "MOV"),

    ]
}