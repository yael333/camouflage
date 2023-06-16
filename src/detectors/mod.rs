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

pub use self::archive::*;
pub use self::audio::*;
pub use self::binary::*;
pub use self::code::*;
pub use self::doc::*;
pub use self::font::*;
pub use self::image::*;
pub use self::markup::*;
pub use self::odf::*;
pub use self::video::*;

use std::error::Error;

pub trait Detector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>>;
}

// Function to return a list of all available detectors
pub fn get_available_detectors() -> Vec<(Box<dyn Detector + 'static>, &'static str)> {
    vec![
        // Archive
        (Box::new(z7Detector), "7Z"),
        (Box::new(ArDetector), "AR"),
        (Box::new(Bz2Detector), "BZ2"),
        (Box::new(CabDetector), "CAB"),
        (Box::new(CpioDetector), "CPIO"),
        (Box::new(CrxDetector), "CRX"),
        (Box::new(DcmDetector), "DCM"),
        (Box::new(DebDetector), "DEB"),
        (Box::new(EotDetector), "EOT"),
        (Box::new(EpubDetector), "EPUB"),
        (Box::new(GzDetector), "GZ"),
        (Box::new(LzDetector), "LZ"),
        (Box::new(MsiDetector), "MSI"),
        (Box::new(PdfDetector), "PDF"),
        (Box::new(PsDetector), "PS"),
        (Box::new(RarDetector), "RAR"),
        (Box::new(RpmDetector), "RPM"),
        (Box::new(RtfDetector), "RTF"),
        (Box::new(SqliteDetector), "SQLITE"),
        (Box::new(SwfDetector), "SWF"),
        (Box::new(TarDetector), "TAR"),
        (Box::new(XzDetector), "XZ"),
        (Box::new(ZDetector), "Z"),
        (Box::new(ZipDetector), "ZIP"),
        (Box::new(ZstDetector), "ZST"),
        
        // Audio
        (Box::new(AacDetector), "AAC"),
        (Box::new(AiffDetector), "AIFF"),
        (Box::new(AmrDetector), "AMR"),
        (Box::new(ApeDetector), "APE"),
        (Box::new(DsfDetector), "DSF"),
        (Box::new(FlacDetector), "FLAC"),
        (Box::new(M4aDetector), "M4A"),
        (Box::new(MidiDetector), "MIDI"),
        (Box::new(Mp3Detector), "MP3"),
        (Box::new(OggDetector), "OGG"),
        (Box::new(Ogg_opusDetector), "OGG_OPUS"),
        (Box::new(WavDetector), "WAV"),
        
        // Binary
        (Box::new(CoffDetector), "COFF"),
        (Box::new(Coff_i386Detector), "COFF_I386"),
        (Box::new(Coff_ia64Detector), "COFF_IA64"),
        (Box::new(Coff_x64Detector), "COFF_X64"),
        (Box::new(DerDetector), "DER"),
        (Box::new(DexDetector), "DEX"),
        (Box::new(DeyDetector), "DEY"),
        // (Box::new(DllDetector), "DLL"), // Same as EXE
        (Box::new(ElfDetector), "ELF"),
        (Box::new(ExeDetector), "EXE"),
        (Box::new(JavaDetector), "JAVA"),
        (Box::new(LlvmDetector), "LLVM"),
        (Box::new(MachDetector), "MACH"),
        (Box::new(NesDetector), "NES"),
        (Box::new(PemDetector), "PEM"),
        (Box::new(WasmDetector), "WASM"),

        // Code
        (Box::new(JsDetector), "JS"),

        // Doc
        (Box::new(DocDetector), "DOC"),
        (Box::new(DocxDetector), "DOCX"),
        (Box::new(PptDetector), "PPT"),
        (Box::new(PptxDetector), "PPTX"),
        (Box::new(XlsDetector), "XLS"),
        (Box::new(XlsxDetector), "XLSX"),

        // Font
        (Box::new(OtfDetector), "OTF"),
        (Box::new(TtfDetector), "TTF"),
        (Box::new(WoffDetector), "WOFF"),
        (Box::new(Woff2Detector), "WOFF2"),

        // Image
        (Box::new(AvifDetector), "AVIF"),
        (Box::new(BmpDetector), "BMP"),
        (Box::new(Cr2Detector), "CR2"),
        (Box::new(GifDetector), "GIF"),
        (Box::new(HeifDetector), "HEIF"),
        (Box::new(IcoDetector), "ICO"),
        (Box::new(JpegDetector), "JPEG"),
        (Box::new(Jpeg2000Detector), "JPEG2000"),
        (Box::new(JxlDetector), "JXL"),
        (Box::new(JxrDetector), "JXR"),
        (Box::new(OraDetector), "ORA"),
        (Box::new(PngDetector), "PNG"),
        (Box::new(PsdDetector), "PSD"),
        (Box::new(TiffDetector), "TIFF"),
        (Box::new(WebpDetector), "WEBP"),

        // Markup
        (Box::new(HtmlDetector), "HTML"),
        (Box::new(ShellscriptDetector), "SHELLSCRIPT"),
        (Box::new(XmlDetector), "XML"),

        // ODF
        (Box::new(OdpDetector), "ODP"),
        (Box::new(OdsDetector), "ODS"),
        (Box::new(OdtDetector), "ODT"),

        // Video
        (Box::new(AviDetector), "AVI"),
        (Box::new(FlvDetector), "FLV"),
        (Box::new(M4vDetector), "M4V"),
        (Box::new(MkvDetector), "MKV"),
        (Box::new(MovDetector), "MOV"),
        (Box::new(Mp4Detector), "MP4"),
        (Box::new(MpegDetector), "MPEG"),
        (Box::new(WebmDetector), "WEBM"),
        (Box::new(WmvDetector), "WMV"),

    ]
}