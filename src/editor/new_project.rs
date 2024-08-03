use bevy::prelude::*;
use crate::gui::ui_components::FileButtonsAction;
use crate::editor::create_project;

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