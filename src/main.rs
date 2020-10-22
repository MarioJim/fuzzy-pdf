use std::convert::TryFrom;
use std::sync::{Arc, RwLock};

#[macro_use]
extern crate lazy_static;
use rayon::iter::{ParallelBridge, ParallelIterator};
use skim::{
    prelude::{unbounded, SkimOptionsBuilder},
    Skim, SkimItemReceiver, SkimItemSender,
};

/// `Action` and its implementations
mod action;
/// `clap` configuration
mod cli;
/// `Config` struct and its implementations
mod config;
/// `FileIter` enum and its implementations
mod file_iter;
/// `PDFContent` and its implementations
mod pdf;

use action::Action;
use file_iter::FileIter;
use pdf::PDFContent;

lazy_static! {
    /// Global configuration for the application
    pub static ref CONFIG: RwLock<config::Config> = RwLock::new(config::Config::new());
}

fn main() {
    let matches = cli::get_app().get_matches();
    CONFIG.write().unwrap().modify_with_argmatches(&matches);
    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();

    let path = matches.value_of("PATH").unwrap();
    let with_hidden_files = matches.is_present("hidden");

    FileIter::new(path, with_hidden_files)
        .par_bridge()
        .filter_map(|possible_entry| {
            let possible_pdf = possible_entry.ok()?.into_path();
            if possible_pdf.extension()?.to_str()? == "pdf" {
                Some(possible_pdf.into_os_string())
            } else {
                None
            }
        })
        .filter_map(|pdf_path| match PDFContent::try_from(pdf_path) {
            Ok(pdf_content) => Some(pdf_content),
            Err((error, file_path)) => {
                if !CONFIG.read().unwrap().quiet {
                    println!("{:?}: {:?}", file_path, error);
                }
                None
            }
        })
        .for_each_with(tx_item, |tx_item, pdf_content| {
            let _ = tx_item.send(Arc::new(pdf_content));
        });

    let skim_options = SkimOptionsBuilder::default()
        .reverse(true)
        .exact(true)
        .preview_window(Some("down:80%"))
        .preview(Some(""))
        .build()
        .unwrap();

    match Skim::run_with(&skim_options, Some(rx_item)) {
        Some(sk_output) => {
            if sk_output.is_abort {
                std::process::exit(130)
            }

            Action::from_matches(&matches).execute(sk_output);
        }
        None => std::process::exit(1),
    }
}
