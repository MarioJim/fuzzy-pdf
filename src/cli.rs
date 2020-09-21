use clap::{crate_version, App, Arg};

pub fn get_app() -> clap::App<'static> {
    let default_exec = if cfg!(windows) {
        "start"
    } else if cfg!(macos) {
        "open"
    } else {
        "xdg-open"
    };

    App::new("fuzzy-pdf")
        .version(crate_version!())
        .author("MarioJim <mario.emilio.j@gmail.com>")
        .about("Fuzzy finder for a collection of pdf files")
        .arg(
            Arg::new("PATH")
                .about("The path to recursively search for pdf files")
                .default_value(".")
                .index(1),
        )
        .arg(
            Arg::new("COMMAND")
                .about("The command to execute when an item has been selected")
                .default_value(default_exec)
                .index(2),
        )
        .arg(
            Arg::new("context")
                .about("Surrounding lines to show in the preview")
                .short('c')
                .long("context")
                .default_value("3")
                .takes_value(true),
        )
        .arg(
            Arg::new("hidden")
                .about("Search hidden files also")
                .short('H')
                .long("hidden"),
        )
}
