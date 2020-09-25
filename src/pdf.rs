use std::borrow::Cow;
use std::convert::TryFrom;
use std::ffi::OsString;
use std::fmt;
use std::sync::atomic::Ordering;

use grep::printer::{ColorSpecs, StandardBuilder};
use grep::regex::RegexMatcherBuilder;
use grep::searcher::SearcherBuilder;
use poppler::PopplerDocument;
use skim::{AnsiString, ItemPreview, Previewer, SkimItem};
use termcolor::Ansi;

/// Maps a file path to its text content
#[derive(Debug)]
pub struct PDFContent {
    pub file_path: OsString,
    pub content: String,
}

impl SkimItem for PDFContent {
    /// Returns the file path as the entry to be displayed in the item list
    fn display(&self) -> Cow<AnsiString> {
        Cow::Owned(self.file_path.to_str().unwrap().into())
    }

    /// Returns the text contect as the text to be searched on
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.content)
    }

    /// Using `ripgrep` internal components, prints a preview of the content
    /// that matches the query with a context of 3 lines before and after
    fn preview(&self, previewer: &Previewer) -> ItemPreview {
        let matcher = RegexMatcherBuilder::new()
            .case_smart(true)
            .build(previewer.prev_query.as_ref().unwrap())
            .unwrap();
        let width = previewer.width.load(Ordering::Relaxed) as u64;
        let mut printer = StandardBuilder::new()
            .stats(false)
            .color_specs(ColorSpecs::default_with_color())
            .max_columns(Some(width))
            .max_columns_preview(true)
            .build(Ansi::new(vec![]));
        let mut searcher = SearcherBuilder::new()
            .line_number(false)
            .after_context(3)
            .before_context(3)
            .build();
        let _ = searcher.search_slice(&matcher, &self.content.as_bytes(), printer.sink(&matcher));

        ItemPreview::AnsiText(String::from_utf8(printer.into_inner().into_inner()).unwrap())
    }
}

/// Tries to read the pdf's text content using poppler-rs given the file path
impl TryFrom<OsString> for PDFContent {
    type Error = (PopplerError, OsString);

    fn try_from(file_path: OsString) -> Result<Self, Self::Error> {
        let pdf_doc = match PopplerDocument::new_from_file(&file_path, "") {
            Ok(pdf_doc) => pdf_doc,
            Err(_) => return Err((PopplerError::NotAPDF, file_path)),
        };

        let content: String = (0..pdf_doc.get_n_pages())
            .filter_map(|page_idx| pdf_doc.get_page(page_idx))
            .filter_map(|page| page.get_text().map(String::from))
            .collect();

        if content.chars().all(|ch| ch.is_whitespace()) {
            Err((PopplerError::EmptyFile, file_path))
        } else {
            Ok(PDFContent { file_path, content })
        }
    }
}

/// Errors that can occur while parsing the pdf file
pub enum PopplerError {
    /// Poppler couldn't recognize the file as a PDF document
    NotAPDF,
    /// Poppler returned either an empty string or only whitespace chararcters
    EmptyFile,
}

impl fmt::Debug for PopplerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PopplerError::NotAPDF => write!(f, "file couldn't be read as a pdf"),
            PopplerError::EmptyFile => write!(f, "no text could be recognized from this file"),
        }
    }
}
