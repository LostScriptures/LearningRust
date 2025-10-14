//! Read the menues file and use the data to format the Menue for the CLI

use crate::{Progress::*, to_do::AppData, to_do::Progress};
use serde_json::*;
use std::{fs::File, io::Read, process};

/// Saves the loaded information from the menues.json file
pub struct Display {
    options: Value,
    titles: Value,
}

impl Display {
    fn new() -> Display {
        Display {
            options: Value::Null,
            titles: Value::Null,
        }
    }

    /// Loads the data from the menues file into the Display struct
    fn load_menues_file() {
        let filepath = "menues.json";
        let mut buffer = String::new();
        let mut display = Display::new();

        let mut file = match File::open(filepath) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("An error occured while reading menues.json: {e}");
                process::exit(1);
            }
        };

        if let Err(e) = file.read_to_string(&mut buffer) {
            eprintln!("An error occured while reading menues.json: {e}");
            process::exit(1);
        }

        let mut json: Value = match serde_json::from_str(&buffer.as_str()) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("An error occured while parsing menues.json: {e}");
                process::exit(1);
            }
        };

        display.options = json["options"].take();
        display.titles = json["titles"].take();
    }

    /// Fills out the rest of the Display struct (evaluate if needed / not implemented yet)
    pub fn init_display() -> Display {}
}
// Prints a single line of a task
pub fn print_task(task: Progress) -> String {
    let completion = match &task {
        ToDo(_) => "[ ]",
        InProgress(_) => "[.]",
        Done(_) => "[X]",
    };
    let title = &task.get().title;

    format!("{} {}", completion, title)
}

// Prints the remaineder of tasks to complete
pub fn print_remainder(data: &AppData) -> String {
    let total = data.get_task_count();
    let completed = data.get_task_count_finished();

    format!("{} Completed / {} Total", completed, total)
}

// Prints the line of options, also handles I/O
pub fn print_options(options: &str) -> String {}

// Prints the title for the current page
pub fn print_title() {
    todo!();
}
