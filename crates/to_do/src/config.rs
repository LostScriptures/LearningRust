use std::{collections::HashMap, fs};

/// Stores the App configuration parameters
pub struct Config<'a> {
    filepath: &'a str,
    settings: HashMap<&'a str, &'a str>,
}

impl Config<'_> {
    /// Loads the config from the config file
    pub fn load(&mut self) {
        todo!();
    }

    /// Saves the config to the config file
    pub fn save(&self) {
        fs::write(
            self.filepath,
            self.settings
                .iter()
                .map(|(&a, &b)| {
                    let mut str = String::from(a);
                    str.push_str(":");
                    str.push_str(b);
                    str.push_str("\n");
                    str
                })
                .collect::<String>(),
        );
        todo!();
    }
}
