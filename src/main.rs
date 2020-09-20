use std::convert::TryFrom;
use std::process::Command;
use std::sync::Arc;

#[macro_use]
extern crate lazy_static;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use skim::prelude::{unbounded, SkimOptionsBuilder};
use skim::{Skim, SkimItemReceiver, SkimItemSender};
use walkdir::WalkDir;

mod cli;
mod pdf;

fn main() {
    let matches = cli::get_app().get_matches();

    let file_paths: Vec<String> = WalkDir::new(matches.value_of("PATH").unwrap())
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path().into_os_string().into_string().unwrap())
        .filter(|e| e.ends_with(".pdf"))
        .collect();
    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();

    file_paths
        .par_iter()
        .filter_map(|file_path| match pdf::PDFContent::try_from(file_path) {
            Ok(pdf_content) => Some(pdf_content),
            Err(error) => {
                println!("{}: {:?}", file_path, error);
                None
            }
        })
        .for_each(|pdf_content| {
            let _ = tx_item.send(Arc::new(pdf_content));
        });
    drop(tx_item);

    let preview_cmd = format!(
        "echo {{}} | grep -E {{q}} --ignore-case --context={} --color=always",
        matches.value_of("context").unwrap_or("3")
    );
    let skim_options = SkimOptionsBuilder::default()
        .reverse(true)
        .exact(true)
        .preview_window(Some("down:80%"))
        .preview(Some(&preview_cmd))
        .build()
        .unwrap();

    let selected_items = Skim::run_with(&skim_options, Some(rx_item))
        .map(|elem| elem.selected_items)
        .unwrap_or_else(|| Vec::new());

    // TODO: Implement the possibility to inject a complex command like
    // https://github.com/lotabout/skim/blob/master/src/util.rs#L332
    for selected_item in selected_items {
        let file_path = &(*selected_item)
            .as_any()
            .downcast_ref::<pdf::PDFContent>()
            .unwrap()
            .filename;
        let _ = Command::new(matches.value_of("COMMAND").unwrap())
            .arg(file_path)
            .spawn()
            .unwrap();
    }
}
