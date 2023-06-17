mod archive;
mod audio;
mod binary;
mod code;
mod doc;
mod font;
mod image;
mod markup;
mod odf;
mod rom;
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
pub use self::rom::*;
pub use self::video::*;

use std::error::Error;

pub trait Detector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>>;
}

pub trait Validator {
    fn validate(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(true)
    }
}

pub trait DetectorValidator: Detector + Validator {}

impl<T: Detector + Validator> DetectorValidator for T {}

// Function to return a list of all available detectors
pub fn get_available_detectors() -> Vec<(Box<dyn DetectorValidator + 'static>, &'static str)> {
    vec![
        // Archive
        (Box::new(Z7Format), "7Z"),
        (Box::new(ArFormat), "AR"),
        (Box::new(Bz2Format), "BZ2"),
        (Box::new(CabFormat), "CAB"),
        (Box::new(CpioFormat), "CPIO"),
        (Box::new(CrxFormat), "CRX"),
        (Box::new(DcmFormat), "DCM"),
        (Box::new(DebFormat), "DEB"),
        (Box::new(EotFormat), "EOT"),
        (Box::new(EpubFormat), "EPUB"),
        (Box::new(GzFormat), "GZ"),
        (Box::new(LzFormat), "LZ"),
        (Box::new(MsiFormat), "MSI"),
        (Box::new(PdfFormat), "PDF"),
        (Box::new(PsFormat), "PS"),
        (Box::new(RarFormat), "RAR"),
        (Box::new(RpmFormat), "RPM"),
        (Box::new(RtfFormat), "RTF"),
        (Box::new(SqliteFormat), "SQLITE"),
        (Box::new(SwfFormat), "SWF"),
        (Box::new(TarFormat), "TAR"),
        (Box::new(XzFormat), "XZ"),
        (Box::new(ZFormat), "Z"),
        (Box::new(ZipFormat), "ZIP"),
        (Box::new(ZstFormat), "ZST"),
        
        // Audio
        (Box::new(AacFormat), "AAC"),
        (Box::new(AiffFormat), "AIFF"),
        (Box::new(AmrFormat), "AMR"),
        (Box::new(ApeFormat), "APE"),
        (Box::new(DsfFormat), "DSF"),
        (Box::new(FlacFormat), "FLAC"),
        (Box::new(M4aFormat), "M4A"),
        (Box::new(MidiFormat), "MIDI"),
        (Box::new(Mp3Format), "MP3"),
        (Box::new(OggFormat), "OGG"),
        (Box::new(Ogg_opusFormat), "OGG_OPUS"),
        (Box::new(WavFormat), "WAV"),
        
        // Binary
        (Box::new(CoffFormat), "COFF"),
        (Box::new(Coff_i386Format), "COFF_I386"),
        (Box::new(Coff_ia64Format), "COFF_IA64"),
        (Box::new(Coff_x64Format), "COFF_X64"),
        (Box::new(DerFormat), "DER"),
        (Box::new(DexFormat), "DEX"),
        (Box::new(DeyFormat), "DEY"),
        // (Box::new(DllFormat), "DLL"), // Same as EXE
        (Box::new(ElfFormat), "ELF"),
        (Box::new(ExeFormat), "EXE"),
        (Box::new(JavaFormat), "JAVA"),
        (Box::new(LlvmFormat), "LLVM"),
        (Box::new(MachFormat), "MACH"),
        (Box::new(PemFormat), "PEM"),
        (Box::new(WasmFormat), "WASM"),

        // Code
        (Box::new(JsFormat), "JS"),

        // Doc
        (Box::new(DocFormat), "DOC"),
        (Box::new(DocxFormat), "DOCX"),
        (Box::new(PptFormat), "PPT"),
        (Box::new(PptxFormat), "PPTX"),
        (Box::new(XlsFormat), "XLS"),
        (Box::new(XlsxFormat), "XLSX"),

        // Font
        (Box::new(OtfFormat), "OTF"),
        (Box::new(TtfFormat), "TTF"),
        (Box::new(WoffFormat), "WOFF"),
        (Box::new(Woff2Format), "WOFF2"),

        // Image
        (Box::new(AvifFormat), "AVIF"),
        (Box::new(BmpFormat), "BMP"),
        (Box::new(Cr2Format), "CR2"),
        (Box::new(GifFormat), "GIF"),
        (Box::new(HeifFormat), "HEIF"),
        (Box::new(IcoFormat), "ICO"),
        (Box::new(JpegFormat), "JPEG"),
        (Box::new(Jpeg2000Format), "JPEG2000"),
        (Box::new(JxlFormat), "JXL"),
        (Box::new(JxrFormat), "JXR"),
        (Box::new(OraFormat), "ORA"),
        (Box::new(PngFormat), "PNG"),
        (Box::new(PsdFormat), "PSD"),
        (Box::new(TiffFormat), "TIFF"),
        (Box::new(WebpFormat), "WEBP"),

        // Markup
        (Box::new(HtmlFormat), "HTML"),
        (Box::new(ShellscriptFormat), "SHELLSCRIPT"),
        (Box::new(XmlFormat), "XML"),

        // ODF
        (Box::new(OdpFormat), "ODP"),
        (Box::new(OdsFormat), "ODS"),
        (Box::new(OdtFormat), "ODT"),

        // ROM
        (Box::new(NesFormat), "NES"),
        (Box::new(GbFormat), "GB"),

        // Video
        (Box::new(AviFormat), "AVI"),
        (Box::new(FlvFormat), "FLV"),
        (Box::new(M4vFormat), "M4V"),
        (Box::new(MkvFormat), "MKV"),
        (Box::new(MovFormat), "MOV"),
        (Box::new(Mp4Format), "MP4"),
        (Box::new(MpegFormat), "MPEG"),
        (Box::new(WebmFormat), "WEBM"),
        (Box::new(WmvFormat), "WMV"),
    ]
}