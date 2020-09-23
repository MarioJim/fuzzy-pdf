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

#[derive(Debug)]
pub struct PDFContent {
    pub filename: OsString,
    pub content: String,
}

impl SkimItem for PDFContent {
    fn display(&self) -> Cow<AnsiString> {
        Cow::Owned(self.filename.to_str().unwrap().into())
    }

    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.content)
    }

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

impl TryFrom<OsString> for PDFContent {
    type Error = (PopplerError, OsString);

    fn try_from(filename: OsString) -> Result<Self, Self::Error> {
        let pdf_doc = match PopplerDocument::new_from_file(&filename, "") {
            Ok(pdf_doc) => pdf_doc,
            Err(_) => return Err((PopplerError::NotAPDF, filename)),
        };

        let content: String = (0..pdf_doc.get_n_pages())
            .filter_map(|page_idx| pdf_doc.get_page(page_idx))
            .filter_map(|page| page.get_text().map(String::from))
            .collect();

        if content.chars().all(|ch| ch.is_whitespace()) {
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
            PopplerError::NotAPDF => write!(f, "file couldn't be read as a pdf"),
            PopplerError::EmptyFile => write!(f, "no text could be recognized from this file"),
        }
    }
}
