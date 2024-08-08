use bevy::{prelude::*, render::camera::RenderTarget, window::WindowRef};
use crate::gui::ui_components::FileButtonsAction;
use crate::editor::create_project;
use crate::gui::project_manager;

pub fn open_project_creator_system(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &FileButtonsAction), (Changed<Interaction>, With<Button>)>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed && matches!(action, FileButtonsAction::New) {
            let project_selector_window = commands
                .spawn(Window {
                    title: "Create new project".to_owned(),
                    ..default()
                })
                .id();
            
            let second_window_camera = commands
                .spawn(Camera2dBundle {
                    camera: Camera {
                        target: RenderTarget::Window(WindowRef::Entity(project_selector_window)),
                        ..default()
                    },
                    ..default()
                })
                .id();

        }
    }
}