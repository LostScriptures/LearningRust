use self::Progress::*;
use crate::config::Config;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read, Write},
};

/// Allows the distinction between the differnts stages of completion a task can be in
#[derive(Debug, Clone, PartialEq)]
pub enum Progress {
    ToDo(Task),
    InProgress(Task),
    Done(Task),
}

/// The basic struct that holds the Info of a single task
#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
}

impl Task {
    /// Returns a new ToDo Task instance with filled out information
    fn new(id: i32, title: String, description: String) -> Progress {
        ToDo(Task {
            id,
            description,
            title,
        })
    }

    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        // Write ID
        writer.write_all(&self.id.to_le_bytes())?;

        // Write title
        let title_bytes = self.title.as_bytes();
        let title_len = title_bytes.len() as u32;

        writer.write_all(&title_len.to_le_bytes())?;
        writer.write_all(title_bytes)?;

        // Write description
        let description_bytes = self.description.as_bytes();
        let description_len = description_bytes.len() as u32;

        writer.write_all(&description_len.to_le_bytes())?;
        writer.write_all(description_bytes)?;

        Ok(())
    }

    fn read_from<R: Read>(reader: &mut R) -> io::Result<Self> {
        // Read ID
        let mut buf4 = [0u8; 4];
        reader.read_exact(&mut buf4)?;

        let id = i32::from_le_bytes(buf4);

        // Read title
        reader.read_exact(&mut buf4)?;
        let title_len = u32::from_le_bytes(buf4) as usize;

        let mut title_buf = vec![0u8; title_len];
        reader.read_exact(&mut title_buf)?;
        let title = String::from_utf8(title_buf)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"))?;

        // Write description
        reader.read_exact(&mut buf4)?;
        let description_len = u32::from_le_bytes(buf4) as usize;

        let mut description_buf = vec![0u8; description_len];
        reader.read_exact(&mut description_buf)?;
        let description = String::from_utf8(description_buf)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"))?;

        Ok(Task {
            id,
            title,
            description,
        })
    }

    /// Clears title and sets new title
    fn change_title(&mut self, new_title: &str) {
        self.title.clear();
        self.title.push_str(new_title);
    }

    /// Clears description and sets new description
    fn change_description(&mut self, new_description: &str) {
        self.description.clear();
        self.description.push_str(new_description);
    }
}

/// Holds the current relevant information (App configuration and loaded Tasks)
#[derive(Debug)]
pub struct AppData {
    tasks: HashMap<i32, Progress>,
    pub config: Config,
}

impl AppData {
    pub fn new() -> AppData {
        AppData {
            tasks: HashMap::new(),
            config: Config::new(),
        }
    }
    /// Loads the saved Tasks into the tasks HashMap
    pub fn load(&mut self) -> io::Result<()> {
        let mut file = File::open("data.bin")?;
        let mut buf = [0u8; 4];

        file.read_exact(&mut buf)?;
        let count = u32::from_le_bytes(buf);

        for _ in 0..count {
            // Read key
            file.read_exact(&mut buf)?;
            let key = i32::from_le_bytes(buf);

            // Read Progress identifier
            let mut ident = [0u8];
            file.read_exact(&mut ident)?;

            // Read Task and wrap in progress
            let task = Task::read_from(&mut file)?;
            let task = match ident {
                [1u8] => ToDo(task),
                [2u8] => InProgress(task),
                [3u8] => Done(task),
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid Progress identifier in save",
                    ));
                }
            };
            self.tasks.insert(key, task);
        }

        Ok(())
    }

    /// Saves the saves tasks into a file
    pub fn save(&self) -> io::Result<()> {
        let mut file = File::create("data.bin")?;
        let len = self.tasks.len() as u32;
        file.write_all(&len.to_le_bytes())?;

        for (k, v) in &self.tasks {
            // Write key
            file.write_all(&k.to_le_bytes())?;

            // Write Progress identifier
            let (ident, task) = match v {
                ToDo(t) => (1u8, t),
                InProgress(t) => (2u8, t),
                Done(t) => (3u8, t),
            };

            file.write_all(&ident.to_le_bytes())?;
            task.write_to(&mut file)?;
        }

        Ok(())
    }

    /// Saves the apps config
    pub fn save_config(&self) {
        self.config.save();
    }

    /// Loads the apps config
    pub fn load_config(&mut self) -> io::Result<()> {
        self.config.load()?;
        Ok(())
    }

    /// Adding a task to the tasks HashMap
    /// The IDs are handeled automatically
    pub fn add_task(&mut self, title: String, description: String) {
        let id = match self.tasks.keys().max() {
            Some(id) => {
                if *id == i32::MAX {
                    // Search for free ID
                    let mut ptr = 0;
                    while self.tasks.contains_key(&ptr) {
                        ptr += 1;
                    }
                    ptr
                } else {
                    id + 1
                }
            }
            None => 0,
        };

        self.tasks.insert(id, Task::new(id, title, description));
    }

    /// Deleting a task from the HashMap
    /// # Result
    /// Returns `Err` if there was no entry with the given id
    /// Returns `Ok` containing the deleted item
    pub fn del_task(&mut self, id: i32) -> Result<Progress, ()> {
        if let Some(item) = self.tasks.remove(&id) {
            return Ok(item);
        } else {
            return Err(());
        }
    }

    /// Get a reference to the task HashMap
    pub fn get_tasks(&self) {
        todo!();
    }

    /// Get a specific task by id
    /// # Result
    /// `Ok`: Returns the Progress with the contained task
    /// `Err`: Returns if the task couldn't be found
    pub fn get_task_by_id(&self, id: i32) -> Result<Progress, ()> {
        if let Some(p) = self.tasks.get(&id) {
            Ok(p.clone())
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_task() {
        let mut data = AppData::new();
        let compare = Task::new(
            0,
            String::from("test"),
            String::from("This is a test description"),
        );

        data.add_task(
            String::from("test"),
            String::from("This is a test description"),
        );
        assert_eq!(compare, data.get_task_by_id(0).unwrap());
    }

    #[test]
    fn save_tasks() {
        let mut data = AppData::new();

        data.add_task(
            String::from("test"),
            String::from("This is a test description"),
        );

        assert!(data.save().is_ok());
    }

    #[test]
    fn load_tasks() {
        let mut data = AppData::new();
        let compare = Task::new(
            0,
            String::from("test"),
            String::from("This is a test description"),
        );

        assert!(data.load().is_ok());
        assert_eq!(data.get_task_by_id(0).unwrap(), compare);
    }
}
