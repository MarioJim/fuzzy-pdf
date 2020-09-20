use std::borrow::Cow;
use std::convert::TryFrom;
use std::ffi::OsString;
use std::fmt;
use std::process::Command;

use regex::Regex;
use skim::{AnsiString, SkimItem};

#[derive(Debug)]
pub struct PDFContent {
    pub filename: OsString,
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

impl TryFrom<OsString> for PDFContent {
    type Error = (PDFToTextError, OsString);

    fn try_from(filename: OsString) -> Result<Self, Self::Error> {
        let pdftotext_result = match Command::new("pdftotext")
            .arg("-nopgbrk")
            .arg(&filename)
            .arg("-")
            .output()
        {
            Ok(out) => out.stdout,
            Err(_) => return Err((PDFToTextError::FailedToExecute, filename)),
        };

        let content = match String::from_utf8(pdftotext_result) {
            Ok(content) => content,
            Err(_) => return Err((PDFToTextError::FailedToCreateString, filename)),
        };

        lazy_static! {
            static ref ONLY_WHITESPACE: Regex = Regex::new(r"^\s*$").unwrap();
        }
        if ONLY_WHITESPACE.is_match(&content) {
            Err((PDFToTextError::EmptyFile, filename))
        } else {
            Ok(PDFContent { filename, content })
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
