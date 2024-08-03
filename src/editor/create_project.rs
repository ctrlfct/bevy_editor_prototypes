use rfd::{FileDialog, MessageDialog, MessageDialogResult};
use std::fs;
use std::path::{Path, PathBuf};
use crate::editor::init_project;

/// Function to create a new project in the selected directory
pub fn create_new_project(project_name: &str) -> Option<PathBuf> {
    if let Some(project_path) = FileDialog::new()
        .set_title("Select Directory for New Project")
        .pick_folder()
    {
        let full_path = project_path.join(project_name);

        if fs::create_dir(&full_path).is_err() {
            eprintln!("Failed to create project directory at {:?}", full_path);
            return None;
        }

        println!("Creating new project at: {:?}", full_path);

        if init_project::init_bevy_project(&full_path) {
            println!("Created new Bevy project at: {:?}", full_path);
            return Some(full_path);
        } else {
            println!("Failed to initialize Bevy project");
        }
    } else {
        println!("No directory selected for the new project");
    }

    None
}