use std::fs;
use std::io;

pub fn read_username() -> Result<String, io::Error> {
    // let mut username_file = File::open("username.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    // Shortend version
    // let mut username = String::new();
    // File::open("username.txt")?.read_to_string(&mut username)?;
    // Ok(username)

    // Even shorter
    fs::read_to_string("username.txt")
}

pub fn last_char_of_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
