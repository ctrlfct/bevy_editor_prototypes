use bevy::prelude::*;
use crate::editor_settings::EditorSettings;

mod toolbar;
mod button;
mod editor_settings;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn init_editor_settings(mut commands: Commands, asset_server: Res<AssetServer>) {
    let settings = EditorSettings::new(&asset_server);
    commands.insert_resource(settings);
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init_editor_settings)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, toolbar::setup_toolbar.after(init_editor_settings))
        .run()
}
