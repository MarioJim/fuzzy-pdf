use std::convert::TryFrom;

use rayon::prelude::*;
use skim::prelude::*;
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
        .map(|file_path| pdf::PDFContent::try_from(file_path).unwrap())
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
