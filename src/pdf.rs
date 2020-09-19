use std::convert::TryFrom;
use std::fmt;
use std::process::Command;

#[derive(Debug)]
pub struct PDFRepr {
    pub filename: String,
    pub content: String,
}

pub enum PDFToTextError {
    FailedToExecute,
    FailedToCreateString,
}

impl fmt::Debug for PDFToTextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PDFToTextError::FailedToExecute => write!(f, "pdftotext binary failed to execute"),
            PDFToTextError::FailedToCreateString => {
                write!(f, "failed to create a string from pdftotext output")
            }
        }
    }
}

impl<'a> TryFrom<&'a str> for PDFRepr {
    type Error = PDFToTextError;

    fn try_from(filename: &'a str) -> Result<Self, Self::Error> {
        let pdftotext_result = Command::new("pdftotext")
            .arg("-nopgbrk")
            .arg(filename)
            .arg("-")
            .output()
            .map_err(|_| PDFToTextError::FailedToExecute)?
            .stdout;

        let content = String::from_utf8(pdftotext_result)
            .map_err(|_| PDFToTextError::FailedToCreateString)?;

        Ok(PDFRepr {
            filename: String::from(filename),
            content,
        })
    }
}
