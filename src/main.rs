use std::convert::TryFrom;

use rayon::prelude::*;
use skim::prelude::*;
use walkdir::WalkDir;

mod pdf;

fn main() {
    let file_paths: Vec<String> = WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path().into_os_string().into_string().unwrap())
        .filter(|e| e.ends_with(".pdf"))
        .collect();
    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();

    file_paths
        .par_iter()
        .map(|file_path| pdf::PDFContent::try_from(file_path).unwrap())
        .for_each(|pdf_content| {
            let _ = tx_item.send(Arc::new(pdf_content));
        });
    drop(tx_item);

    // TODO: Generate string with clap arguments, like context lines (-NUM in grep cmd)
    let skim_options = SkimOptionsBuilder::default()
        .reverse(true)
        .exact(true)
        .preview_window(Some("down:80%"))
        .preview(Some("echo {} | grep -E {q} -i -3 --color=always"))
        .build()
        .unwrap();

    let selected_items = Skim::run_with(&skim_options, Some(rx_item))
        .map(|elem| elem.selected_items)
        .unwrap_or_else(|| Vec::new());

    for selected_item in selected_items {
        print!(
            "{}",
            (*selected_item)
                .as_any()
                .downcast_ref::<pdf::PDFContent>()
                .unwrap()
                .filename
        );
    }
}
