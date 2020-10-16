use std::convert::TryFrom;
use std::sync::{Arc, RwLock};

#[macro_use]
extern crate lazy_static;
use jwalk::WalkDir;
use skim::prelude::{unbounded, SkimOptionsBuilder};
use skim::{Skim, SkimItemReceiver, SkimItemSender};

/// `Action` and its implementations
mod action;
/// `clap` configuration
mod cli;
/// `Config` struct and its impl
mod config;
/// `PDFContent` and its implementations
mod pdf;

lazy_static! {
    /// Global configuration for the application
    pub static ref CONFIG: RwLock<config::Config> = RwLock::new(config::Config::new());
}

fn main() {
    let matches = cli::get_app().get_matches();
    CONFIG.write().unwrap().modify_with_argmatches(&matches);
    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();

    WalkDir::new(matches.value_of("PATH").unwrap())
        .skip_hidden(!matches.is_present("hidden"))
        .into_iter()
        .filter_map(|possible_path| possible_path.ok())
        .filter_map(|path| {
            let possible_pdf = path.path();
            if let Some(Some("pdf")) = possible_pdf.extension().map(|ext| ext.to_str()) {
                return Some(possible_pdf.into_os_string());
            }
            None
        })
        .filter_map(|pdf_path| match pdf::PDFContent::try_from(pdf_path) {
            Ok(pdf_content) => Some(pdf_content),
            Err((error, file_path)) => {
                if !CONFIG.read().unwrap().quiet {
                    println!("{:?}: {:?}", file_path, error);
                }
                None
            }
        })
        .for_each(|pdf_content| {
            let _ = tx_item.send(Arc::new(pdf_content));
        });
    drop(tx_item);

    let skim_options = SkimOptionsBuilder::default()
        .reverse(true)
        .exact(true)
        .preview_window(Some("down:80%"))
        .preview(Some(""))
        .build()
        .unwrap();

    let arguments =
        Skim::run_with(&skim_options, Some(rx_item)).unwrap_or_else(|| std::process::exit(130));

    action::Action::from_matches(&matches).execute(arguments);
}
