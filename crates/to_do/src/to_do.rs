use self::Progress::*;
use crate::config::Config;
use std::collections::HashMap;

/// Allows the distinction between the differnts stages of completion a task can be in
#[derive(Debug, Clone)]
pub enum Progress {
    ToDo(Task),
    InProgress(Task),
    Done(Task),
}

/// The basic struct that holds the Info of a single task
#[derive(Debug, Clone)]
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
pub struct AppData<'a> {
    tasks: HashMap<i32, Progress>,
    config: Config<'a>,
}

impl AppData<'_> {
    /// Loads the saved Tasks into the tasks HashMap and the config from the files
    pub fn _load(&mut self) {
        self.config.load();
        todo!();
    }

    /// Saves the loaded tasks into a file and the loaded config to the config file
    pub fn _save(&self) {
        self.config.save();
        todo!();
    }

    /// Adding a task to the tasks HashMap
    /// The IDs are handeled automatically
    pub fn _add_task(&mut self, title: String, description: String) {
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
    pub fn _del_task(&mut self, id: i32) -> Result<Progress, ()> {
        if let Some(item) = self.tasks.remove(&id) {
            return Ok(item);
        } else {
            return Err(());
        }
    }

    /// Get a reference to the task HashMap
    pub fn _get_tasks(&self) {
        todo!();
    }

    /// Get a specific task by id
    pub fn _get_task_by_id(&self) {
        todo!();
    }
}
