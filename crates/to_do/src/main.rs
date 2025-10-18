use std::io;

use to_do::*;

mod config;
mod display;
mod to_do;

fn main() {
    let mut appdata = AppData::new();

    // Load config and save
    load(&mut appdata);

    // Start menue
    // Print main menue and options

    // Add Task
    // Print menue
    // Ask for title
    // Ask for description
    // Ask for confirmation
    // Yes: Back to main
    // No: Back to add task

    // Delete Task
    // Print menue
    // Ask for task to delete
    // Ask for confirmation
    // Yes: Back to main
    // No: BAck to Delete

    // Change Status
    // Print menue
    // Ask for task to change
    // Ask for status to change to
    // Back to change status

    // Edit Task
    // Print menue
    // Ask for task to edit
    // Ask for title or description
    // Ask for new value
    // Ask for confirmation
    // Yes: Ask to change tasks status
    // Yes: Go to change status
    // No: back to edit
    // No: back to main

    // Save & Exit
}

fn load(data: &mut AppData) -> io::Result<()> {
    data.config_load();
    data.load()?;
    Ok(())
}

fn save(data: &AppData) -> io::Result<()> {
    data.config_save();
    data.save()?;
    Ok(())
}
