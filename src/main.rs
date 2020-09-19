use std::convert::TryFrom;

use walkdir::WalkDir;

mod pdf;

fn main() {
    let pdf_file_paths = WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_str().unwrap().ends_with(".pdf"))
        .map(|e| e.into_path().into_os_string().into_string().unwrap());

    let pdf_files: Vec<_> = pdf_file_paths
        .map(|string| pdf::PDFRepr::try_from(&string).unwrap())
        .collect();

    for pdf_content in pdf_files {
        print!("{}\t{}\u{0000}", pdf_content.filename, pdf_content.content);
    }
}
