use rfd::FileDialog;
use std::fs;
use std::path::{Path, PathBuf};

/// Function to create a new project in the selected directory
pub fn create_new_project() -> Option<PathBuf> {
    if let Some(selected_dir) = FileDialog::new()
        .set_title("Select Directory for New Project")
        .pick_folder()
    {
        let project_path = selected_dir.join("NewProject");
        if let Err(e) = fs::create_dir(&project_path) {
            eprintln!("Failed to create project directory: {}", e);
            return None;
        }

        println!("Created new project at: {:?}", project_path);
        Some(project_path)
    } else {
        None
    }
}