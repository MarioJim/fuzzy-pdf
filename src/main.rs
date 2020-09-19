use std::str;
use std::{convert::TryFrom, process::Command};

use rayon::prelude::*;

mod pdf;

fn main() {
    let pdf_files = Command::new("fd")
        .arg("--extension=pdf")
        .output()
        .expect("fd failed to execute");
    let lines = str::from_utf8(&pdf_files.stdout).unwrap();

    let p: Vec<pdf::PDFRepr> = lines
        .par_lines()
        .map(|string| pdf::PDFRepr::try_from(string).unwrap())
        .collect();

    for l in p {
        print!("{}\t{}\u{0000}", l.filename, l.content);
    }
}
