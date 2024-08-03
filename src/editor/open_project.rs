use bevy::prelude::*;
use crate::gui::ui_components::FileButtonsAction;
use crate::editor::create_project;
use crate::gui::project_selector;

pub fn open_project_system(
    mut interaction_query: Query<
        (&Interaction, &FileButtonsAction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed && matches!(action, FileButtonsAction::Open) {
            println!("Opened!");
        }
    }
}