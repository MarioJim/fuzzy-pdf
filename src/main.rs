use std::convert::TryFrom;

use rayon::prelude::*;

mod get_files;
mod pdf;

fn main() {
    let pdf_files = get_files::find_pdf_files();

    let p: Vec<pdf::PDFRepr> = pdf_files
        .par_lines()
        .map(|string| pdf::PDFRepr::try_from(string).unwrap())
        .collect();

    for l in p {
        print!("{}\t{}\u{0000}", l.filename, l.content);
    }
}
