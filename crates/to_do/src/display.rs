//! Read the menues file and use the data to format the Menue for the CLI

use crate::to_do::{AppData, Progress};
use serde_json::Value;
use std::{fs::File, io::{self, Read, Write}, process};

/// Saves the loaded information from the menues.json file
pub struct Display {
    options: Value,
    titles: Value,
}

impl Display {
    pub fn new() -> Display {
        Display {
            options: Value::Null,
            titles: Value::Null,
        }
    }

    /// Loads the data from the menues file into the Display struct
    pub fn load_menues_file() -> Display {
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

        let json: Value = match serde_json::from_str(buffer.as_str()) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("An error occured while parsing menues.json: {e}");
                process::exit(1);
            }
        };

        // Safely extract options and titles if present, otherwise leave Null
        display.options = json.get("options").cloned().unwrap_or(Value::Null);
        display.titles = json.get("titles").cloned().unwrap_or(Value::Null);

        display
    }

    // /// Fills out the rest of the Display struct (evaluate if needed / not implemented yet)
    // pub fn init_display() -> Display {}
}

/// Prints a single line of a task
pub fn print_task(task: &Progress) -> String {
    let completion = match task {
        Progress::ToDo(_) => "[ ]",
        Progress::InProgress(_) => "[.]",
        Progress::Done(_) => "[X]",
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

/// Prints the line of options, also handles I/O and returns selected option
pub fn print_options(display: &Display) -> i32 {
    // Expect options to be an array of strings. Show numbered options (1..n).
    match display.options.as_array() {
    Some(arr) if !arr.is_empty() => {
        for (i, val) in arr.iter().enumerate() {
            // Create an owned String so we don't borrow a temporary value
            let text_owned = match val.as_str() {
                Some(s) => s.to_string(),
                None => val.to_string(),
            };
            println!("{}. {}", i + 1, text_owned);
        }

        // Prompt user for selection
        loop {
                print!("Select an option (number, empty to cancel): ");
                // Ensure prompt is flushed
                if let Err(e) = io::stdout().flush() {
                    eprintln!("Failed to flush stdout: {e}");
                }

                let mut input = String::new();
                if let Err(e) = io::stdin().read_line(&mut input) {
                    eprintln!("Failed to read input: {e}");
                    return 0;
                }

                let input = input.trim();
                if input.is_empty() {
                    // treat empty as cancel
                    return 0;
                }

                match input.parse::<i32>() {
                    Ok(n) if n >= 1 && (n as usize) <= arr.len() => return n,
                    _ => {
                        println!("Invalid selection, please enter a number between 1 and {}.", arr.len());
                        continue;
                    }
                }
            }
        }
        _ => {
            // No options available
            println!("No options configured.");
            0
        }
    }
}

// Prints the title for the current page
pub fn print_title(display: &Display, key: &str) {
    // Try the provided key first, then common keys in titles: "main", "title", otherwise print as string if possible
    if let Some(obj) = display.titles.as_object() {
        if let Some(Value::String(s)) = obj.get(key) {
            println!("{}", s);
            return;
        }
        if let Some(Value::String(s)) = obj.get("main") {
            println!("{}", s);
            return;
        }
        if let Some(Value::String(s)) = obj.get("title") {
            println!("{}", s);
            return;
        }
    }

    if let Some(s) = display.titles.as_str() {
        println!("{}", s);
        return;
    }

    // Fallback: print the whole titles value prettified if it's not null
    if !display.titles.is_null() {
        println!("{}", display.titles);
    }
}
