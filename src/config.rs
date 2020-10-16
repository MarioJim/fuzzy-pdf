use clap::ArgMatches;

pub struct Config {
    pub context: usize,
    pub max_pages: usize,
    pub quiet: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            context: 3,
            max_pages: 100,
            quiet: false,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
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
