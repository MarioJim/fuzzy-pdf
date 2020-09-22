use std::convert::TryFrom;
use std::process::Command;
use std::sync::Arc;

#[macro_use]
extern crate lazy_static;
use jwalk::WalkDir;
use skim::prelude::{unbounded, SkimOptionsBuilder};
use skim::{Skim, SkimItemReceiver, SkimItemSender};

mod cli;
mod pdf;

fn main() {
    let matches = cli::get_app().get_matches();
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
            Err((error, filename)) => {
                println!("{:?}: {:?}", filename, error);
                None
            }
        })
        .for_each(|pdf_content| {
            let _ = tx_item.send(Arc::new(pdf_content));
        });
    drop(tx_item);

    let preview_cmd = format!(
        "echo {{}} | grep --ignore-case --context={} --color=always {{q}}",
        matches.value_of("context").unwrap()
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
        .unwrap_or_else(Vec::new);

    // TODO: Implement the possibility to inject a complex command like
    // https://github.com/lotabout/skim/blob/master/src/util.rs#L332
    for selected_item in selected_items {
        let file_path = &(*selected_item)
            .as_any()
            .downcast_ref::<pdf::PDFContent>()
            .unwrap()
            .filename;
        Command::new(matches.value_of("COMMAND").unwrap())
            .arg(file_path)
            .spawn()
            .unwrap();
    }
}
