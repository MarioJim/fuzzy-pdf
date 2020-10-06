use clap::ArgMatches;

pub struct Config {
    pub context: usize,
    pub max_pages: usize,
    pub quiet: bool,
}

impl Config {
    pub fn new() -> Self {
        Config {
            context: 3,
            max_pages: 100,
            quiet: false,
        }
    }

    pub fn modify_with_argmatches(&mut self, matches: &ArgMatches) {
        if let Some(context_str) = matches.value_of("context") {
            if let Ok(context) = context_str.parse() {
                self.context = context;
            }
        }
        if let Some(max_pages_str) = matches.value_of("max-pages") {
            if let Ok(max_pages) = max_pages_str.parse() {
                self.max_pages = max_pages;
            }
        }
        self.quiet = matches.is_present("quiet");
    }
}
