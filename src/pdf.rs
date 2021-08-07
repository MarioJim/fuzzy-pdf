use std::borrow::Cow;
use std::convert::TryFrom;
use std::ffi::OsString;
use std::fmt;
use std::sync::Arc;

use grep::printer::{ColorSpecs, StandardBuilder};
use grep::regex::RegexMatcherBuilder;
use grep::searcher::SearcherBuilder;
use poppler::PopplerDocument;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use skim::{AnsiString, DisplayContext, ItemPreview, PreviewContext, SkimItem};
use termcolor::Ansi;

/// Maps a file path to its text content
#[derive(Debug)]
pub struct PDFContent {
    pub file_path: OsString,
    pub content: String,
}

impl SkimItem for PDFContent {
    /// Returns the file path as the entry to be displayed in the item list
    fn display(&self, _: DisplayContext) -> AnsiString {
        self.file_path.as_os_str().to_str().unwrap().into()
    }

    /// Returns the text contect as the text to be searched on
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.content)
    }

    /// Using `ripgrep` internal components, prints a preview of the content
    /// that matches the query with a context of 3 lines before and after
    fn preview(&self, context: PreviewContext) -> ItemPreview {
        let matcher = RegexMatcherBuilder::new()
            .case_smart(true)
            .build(context.query)
            .unwrap();
        let width = context.width as u64;
        let mut printer = StandardBuilder::new()
            .stats(false)
            .color_specs(ColorSpecs::default_with_color())
            .max_columns(Some(width))
            .max_columns_preview(true)
            .build(Ansi::new(vec![]));
        let context = crate::CONFIG.read().unwrap().context;
        let mut searcher = SearcherBuilder::new()
            .line_number(false)
            .after_context(context)
            .before_context(context)
            .build();
        let _ = searcher.search_slice(&matcher, self.content.as_bytes(), printer.sink(&matcher));

        ItemPreview::AnsiText(String::from_utf8(printer.into_inner().into_inner()).unwrap())
    }
}

/// Tries to read the pdf's text content using poppler-rs given the file path
impl TryFrom<OsString> for PDFContent {
    type Error = (ParsingError, OsString);

    fn try_from(file_path: OsString) -> Result<Self, Self::Error> {
        let document = match PopplerDocument::from_file(&file_path, "") {
            Ok(pdf_doc) => pdf_doc,
            Err(_) => return Err((ParsingError::NotAPDF, file_path)),
        };

        let num_pages = document.n_pages();
        let max_num_pages = crate::CONFIG.read().unwrap().max_pages;
        if max_num_pages != 0 && max_num_pages < num_pages {
            return Err((ParsingError::TooManyPages, file_path));
        }

        let document_arc = Arc::new(document);

        let content: String = (0..num_pages)
            .into_par_iter()
            .map(|page_idx| {
                Arc::clone(&document_arc)
                    .page(page_idx)
                    .map(|page| page.owned_text())
                    .flatten()
                    .unwrap_or_default()
            })
            .collect();

        if content.chars().all(|ch| ch.is_whitespace()) {
            Err((ParsingError::EmptyFile, file_path))
        } else {
            Ok(PDFContent { file_path, content })
        }
    }
}

/// Errors that can occur while parsing the pdf file
pub enum ParsingError {
    /// Poppler couldn't recognize the file as a PDF document
    NotAPDF,
    /// Poppler returned either an empty string or only whitespace chararcters
    EmptyFile,
    /// The PDF has more pages than permitted
    TooManyPages,
}

impl fmt::Debug for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsingError::NotAPDF => write!(f, "file couldn't be read as a pdf"),
            ParsingError::EmptyFile => write!(f, "no text could be recognized from this file"),
            ParsingError::TooManyPages => write!(f, "has too many pages"),
        }
    }
}
