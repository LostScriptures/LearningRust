use std::{collections::HashMap, fs, io};

/// Stores the App configuration parameters
#[derive(Debug)]
pub struct Config {
    pub settings: HashMap<String, String>,
}

impl Config {
    /// Loads the config from the config file
    pub fn load(&mut self) -> io::Result<()> {
        let filepath = "config.txt";

        let contents = fs::read_to_string(filepath)?;
        for line in contents.lines() {
            if line.starts_with("#") {
                continue;
            }

            let mut parts = line.splitn(2, ":");

            if let (Some(attrib), Some(val)) = (parts.next(), parts.next()) {
                self.settings
                    .insert(attrib.trim().to_string(), val.trim().to_string());
            }
        }

        Ok(())
    }

    /// Saves the config to the config file
    pub fn save(&self) {
        let filepath = "config.txt";
        let _ = fs::write(
            filepath,
            self.settings
                .iter()
                .map(|(a, b)| {
                    let mut str = String::from(a);
                    str.push_str(":");
                    str.push_str(b);
                    str.push_str("\n");
                    str
                })
                .collect::<String>(),
        );
    }
}
