use std::borrow::Cow;
use std::convert::TryFrom;
use std::ffi::OsString;
use std::fmt;

use poppler::PopplerDocument;
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
    type Error = (PopplerError, OsString);

    fn try_from(filename: OsString) -> Result<Self, Self::Error> {
        let pdf_doc = match PopplerDocument::new_from_file(&filename, "") {
            Ok(pdf_doc) => pdf_doc,
            Err(_) => return Err((PopplerError::NotAPDF, filename)),
        };

        let mut content = String::new();
        (0..pdf_doc.get_n_pages())
            .filter_map(|page_idx| pdf_doc.get_page(page_idx))
            .for_each(|page| {
                if let Some(text) = page.get_text() {
                    content.push('\n');
                    content.push_str(text);
                }
            });

        lazy_static! {
            static ref ONLY_WHITESPACE: Regex = Regex::new(r"^\s*$").unwrap();
        }
        if ONLY_WHITESPACE.is_match(&content) {
            Err((PopplerError::EmptyFile, filename))
        } else {
            Ok(PDFContent { filename, content })
        }
    }
}

pub enum PopplerError {
    NotAPDF,
    EmptyFile,
}

impl fmt::Debug for PopplerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PopplerError::NotAPDF => write!(f, "pdftotext binary failed to execute"),
            PopplerError::EmptyFile => write!(f, "no text could be recognized from this file"),
        }
    }
}
