use bevy::{prelude::*, render::camera::RenderTarget, window::WindowRef, app::AppExit};
use crate::gui::ui_components::FileButtonsAction;
use crate::editor::create_project;

#[derive(Component)]
pub struct ProjectSelectorWindowVisible(bool);

pub fn close_editor_system(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &FileButtonsAction), (Changed<Interaction>, With<Button>)>,
    asset_server: Res<AssetServer>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed && matches!(action, FileButtonsAction::Close) {
            app_exit_events.send(AppExit::Success);
        }
    }
}