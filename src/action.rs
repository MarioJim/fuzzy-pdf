use std::env;
use std::process::Command;

use clap::ArgMatches;
use regex::{Captures, Regex};
use skim::SkimOutput;

use crate::pdf;

/// The different options to do after selecting an item
pub enum Action {
    PrintResult,
    RunCommand(String),
}

impl Action {
    /// Creates an `Action` from clap's matches
    pub fn from_matches(matches: &ArgMatches) -> Self {
        match matches.value_of("COMMAND").unwrap().trim() {
            "-" => Action::PrintResult,
            cmd => Action::RunCommand(String::from(cmd)),
        }
    }

    /// Executes (and consumes) an Action
    pub fn execute(self, arguments: SkimOutput) {
        match self {
            Action::PrintResult => {
                let file_path = self.inject_arguments(arguments);
                println!("{}", file_path);
            }
            Action::RunCommand(_) => {
                let shell = env::var("SHELL").unwrap_or_else(|_| "sh".to_string());
                let cmd_str = self.inject_arguments(arguments);
                let _ = Command::new(shell).arg("-c").arg(cmd_str).spawn();
            }
        }
    }

    /// Injects arguments from `SkimOutput` into a given string
    fn inject_arguments(self, arguments: SkimOutput) -> String {
        let starting_cmd = match self {
            Action::PrintResult => String::from("{}"),
            Action::RunCommand(cmd) => cmd,
        };
        let file_path = arguments
            .selected_items
            .first()
            .unwrap()
            .as_any()
            .downcast_ref::<pdf::PDFContent>()
            .unwrap()
            .file_path
            .to_str()
            .unwrap();
        let query = arguments.query.as_str();

        let re_fields = Regex::new(r"(\{ *[qf]? *\})").unwrap();
        if re_fields.is_match(&starting_cmd) {
            let injected_cmd = re_fields.replace_all(&starting_cmd, |caps: &Captures| {
                let range = &caps[1];
                let range = &range[1..range.len() - 1];
                let range = range.trim();
                let replacement = match range {
                    "" | "f" => file_path,
                    "q" => query,
                    _ => "",
                };
                format!("'{}'", replacement)
            });
            String::from(injected_cmd)
        } else {
            format!("{} '{}'", starting_cmd, file_path)
        }
    }
}
