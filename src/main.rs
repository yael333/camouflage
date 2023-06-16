mod formats;

use std::error::Error;
use std::fs;
use prettytable::{Table, Row, Cell};
use prettytable::row;
use structopt::StructOpt;
use crate::formats::{Detector, Validator, get_available_detectors};


#[derive(Debug, StructOpt)]
#[structopt(name = "polyglot-detector", about = "Detects polyglot file formats.")]
struct Opt {
    #[structopt(short, long)]
    file_path: String,
    
    #[structopt(short, long)]
    all: bool,
}

fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let data = fs::read(path)?;
    Ok(data)
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let data = read_file(&opt.file_path)?;

    let detectors = get_available_detectors();

    let mut table = Table::new();
    table.add_row(row!["Format", "Is Valid"]);

    for (detector, format) in detectors {
        let is_format = detector.detect(&data, &opt.file_path)?;
        let is_valid  = detector.validate(&data, &opt.file_path)?;

        if opt.all || (is_format && is_valid) {
            table.add_row(Row::new(vec![
                Cell::new(format),
                Cell::new(if is_format { "✓" } else { "x" }),
            ]));
        }
    }

    table.printstd();
    Ok(())
}
