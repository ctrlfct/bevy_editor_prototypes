use rfd::{FileDialog, MessageDialog, MessageDialogResult};
use std::fs;
use std::path::{Path, PathBuf};
use bevy::prelude::*;
use crate::gui::ui_components::FileButtonsAction;
use crate::editor::init_project;
use crate::editor::create_project;
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
            if let Some(project_path) = create_project::create_new_project("NewProject") {
                println!("New project created at: {:?}", project_path);
            } else {
                println!("Failed to create new project or operation was cancelled");
            }
        }
    }
}