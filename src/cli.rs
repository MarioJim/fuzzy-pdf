use clap::{crate_version, App, Arg};

/// Creates a static clap application for parsing the arguments
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
                .long_about(&COMMAND_LONG_ABOUT)
                .default_value(default_exec)
                .index(2),
        )
        .arg(
            Arg::new("hidden")
                .about("Search hidden files also")
                .short('H')
                .long("hidden"),
        )
        .arg(
            Arg::new("context")
                .about("Surrounding lines to show in the preview")
                .short('c')
                .long("context")
                .takes_value(true),
        )
        .arg(
            Arg::new("max-pages")
                .about("Only parse documents with at most this number of pages. Pass '0' to parse documents with any number of pages")
                .short('m')
                .long("max-pages")
                .takes_value(true),
        )
        .arg(
            Arg::new("quiet")
                .about("Omit printing error messages")
                .short('q')
                .long("quiet"),
        )
}

static COMMAND_LONG_ABOUT: &'static str = "After selecting a file, use this option to either:
 - Pass a '-' to print the file path to stdout (pair this with -q option for better results)
 - Pass a string with placeholders to be executed. You can use {} or {f} to pass the file path, and {q} for the query typed into the search box. If you don't use any placeholders, the string will be appended with the file path and executed.

If you don't pass this argument, the program will open the pdf in the system's default pdf viewer, using 'start' for Windows, 'open' for MacOS, and 'xdg-open' for anything else.";
