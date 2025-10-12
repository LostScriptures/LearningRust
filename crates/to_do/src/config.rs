use std::{collections::HashMap, fs, io};

/// Stores the App configuration parameters
#[derive(Debug)]
pub struct Config {
    pub settings: HashMap<String, String>,
}

impl Config {
    /// Creates a new config struct
    pub fn new() -> Config {
        Config {
            settings: HashMap::new(),
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_config_test() {
        let mut conf = Config::new();

        conf.settings.insert(String::from("a"), String::from("1"));
        conf.settings
            .insert(String::from("b"), String::from("true"));
        conf.settings
            .insert(String::from("c"), String::from("test"));

        conf.save();
    }

    #[test]
    fn load_config_test() -> io::Result<()> {
        let mut conf = Config::new();
        let mut compare = HashMap::new();

        compare.insert(String::from("a"), String::from("1"));
        compare.insert(String::from("b"), String::from("true"));
        compare.insert(String::from("c"), String::from("test"));

        conf.load()?;

        assert_eq!(conf.settings, compare);
        Ok(())
    }
}
