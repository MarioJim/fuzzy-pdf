use std::convert::TryFrom;
use std::fmt;
use std::process::Command;

use skim::prelude::*;

#[derive(Debug)]
pub struct PDFContent {
    pub filename: String,
    pub content: String,
}

impl SkimItem for PDFContent {
    fn display(&self) -> Cow<AnsiString> {
        Cow::Owned(self.content.as_str().into())
    }

    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.content)
    }
}

impl TryFrom<&String> for PDFContent {
    type Error = PDFToTextError;

    fn try_from(filename: &String) -> Result<Self, Self::Error> {
        let pdftotext_result = Command::new("pdftotext")
            .arg("-nopgbrk")
            .arg(filename)
            .arg("-")
            .output()
            .map_err(|_| PDFToTextError::FailedToExecute)?
            .stdout;

        let content = String::from_utf8(pdftotext_result)
            .map_err(|_| PDFToTextError::FailedToCreateString)?;

        Ok(PDFContent {
            filename: String::clone(&filename),
            content,
        })
    }
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
