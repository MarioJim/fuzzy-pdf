use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt;
use std::process::Command;

use regex::Regex;
use skim::{AnsiString, SkimItem};

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

        lazy_static! {
            static ref ONLY_WHITESPACE: Regex = Regex::new(r"^\s*$").unwrap();
        }
        if ONLY_WHITESPACE.is_match(&content) {
            Err(PDFToTextError::EmptyFile)
        } else {
            Ok(PDFContent {
                filename: String::clone(&filename),
                content,
            })
        }
    }
}

pub enum PDFToTextError {
    FailedToExecute,
    FailedToCreateString,
    EmptyFile,
}

impl fmt::Debug for PDFToTextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PDFToTextError::FailedToExecute => write!(f, "pdftotext binary failed to execute"),
            PDFToTextError::FailedToCreateString => {
                write!(f, "failed to create a string from pdftotext output")
            }
            PDFToTextError::EmptyFile => write!(f, "no text could be recognized from this file"),
        }
    }
}
