use rfd::{FileDialog, MessageDialog, MessageDialogResult};
use std::fs;
use std::path::{Path, PathBuf};
use bevy::prelude::*;
use crate::gui::ui_components::FileButtonsAction;
use std::process::Command;

#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;

#[cfg(target_family = "windows")]
use std::os::windows::prelude::*;

pub fn new_project_system(
    mut interaction_query: Query<
        (&Interaction, &FileButtonsAction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed && matches!(action, FileButtonsAction::New) {
            if let Some(project_path) = create_new_project() {
                println!("New project created at: {:?}", project_path);
            } else {
                println!("Failed to create new project or operation was cancelled");
            }
        }
    }
}

/// Function to create a new project in the selected directory
pub fn create_new_project() -> Option<PathBuf> {
    let project_name = "NewProject";

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

        let init_output = Command::new("cargo")
            .args(&["init", "--bin", full_path.to_str().unwrap()])
            .output();

        match init_output {
            Ok(output) => {
                if !output.status.success() {
                    eprintln!(
                        "Cargo init failed with error: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                    return None;
                }
            }
            Err(e) => {
                eprintln!("Failed to execute cargo init: {}", e);
                return None;
            }
        }

        let add_output = Command::new("cargo")
            .current_dir(&full_path)
            .args(&["add", "bevy"])
            .output();

        match add_output {
            Ok(output) => {
                if !output.status.success() {
                    eprintln!(
                        "Cargo add bevy failed with error: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                    return None;
                }
            }
            Err(e) => {
                eprintln!("Failed to execute cargo add bevy: {}", e);
                return None;
            }
        }

        #[cfg(target_family = "unix")]
        {
            if let Ok(metadata) = fs::metadata(&full_path) {
                let mut permissions = metadata.permissions();
                permissions.set_mode(0o755);
                if let Err(e) = fs::set_permissions(&full_path, permissions) {
                    eprintln!("Failed to set permissions: {}", e);
                }
            } else {
                eprintln!("Failed to get metadata for path: {:?}", full_path);
            }
        }

        #[cfg(target_family = "windows")]
        {
            Command::new("icacls")
                .args(&[full_path.to_str().unwrap(), "/grant", "Everyone:(OI)(CI)F"])
                .output()
                .expect("Failed to set permissions");
        }

        println!("Created new Bevy project at: {:?}", full_path);
        return Some(full_path);
    } else {
        println!("No directory selected for the new project");
    }

    None
}