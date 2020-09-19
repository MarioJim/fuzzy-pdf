use std::process::Command;

pub fn find_pdf_files() -> String {
    let pdf_files = Command::new("fd")
        .arg("--extension=pdf")
        .output()
        .expect("fd failed to execute");

    String::from_utf8(pdf_files.stdout).unwrap()
}
