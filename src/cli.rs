use clap::{crate_version, App, Arg};

pub fn get_app() -> clap::App<'static> {
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
            Arg::new("context")
                .about("Surrounding lines to show in the preview")
                .short('c')
                .long("context")
                .takes_value(true),
        )
}
