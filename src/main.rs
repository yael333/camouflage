use std::error::Error;
use std::fs;
use prettytable::{Table, Row, Cell};
use prettytable::row;
use structopt::StructOpt;
use crate::detectors::{PngDetector, JpegDetector, Detector};

mod detectors;

#[derive(Debug, StructOpt)]
#[structopt(name = "polyglot-detector", about = "Detects polyglot file formats.")]
struct Opt {
    #[structopt(short, long)]
    file_path: String,
}

fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let data = fs::read(path)?;
    Ok(data)
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let data = read_file(&opt.file_path)?;

    let detectors: Vec<(Box<dyn Detector>, &'static str)> = vec![
        (Box::new(PngDetector), "PNG"),
        (Box::new(JpegDetector), "JPEG"),
    ];

    let mut table = Table::new();
    table.add_row(row!["Format", "Is Valid"]);

    for (detector, format) in detectors {
        let is_format = detector.detect(&data)?;
        table.add_row(Row::new(vec![
            Cell::new(format),
            Cell::new(if is_format { "Yes" } else { "No" }),
        ]));
    }

    table.printstd();
    Ok(())
}
