use bevy::prelude::*;
use crate::editor_settings::EditorSettings;
use crate::toolbar;

#[derive(Component)]
pub struct MainCanvas;

pub fn spawn_main_canvas(commands: &mut Commands, settings: &Res<EditorSettings>) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: settings.panel_background.into(),
                ..default()
            },
            MainCanvas,
        ))
        .id()
}


pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>, settings: Res<EditorSettings>) {
    let canvas_entity = spawn_main_canvas(&mut commands, &settings);
    commands.entity(canvas_entity).with_children(|parent| {
        toolbar::setup_toolbar(parent, &asset_server, &settings);
    });
}
