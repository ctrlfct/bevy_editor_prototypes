use bevy::prelude::*;
use bevy_editor_prototypes::editor_settings::EditorSettings; 

use bevy_editor_prototypes::gui;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn init_editor_settings(mut commands: Commands, asset_server: Res<AssetServer>) {
    let settings = EditorSettings::new(&asset_server);
    commands.insert_resource(settings);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init_editor_settings)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, gui::main_canvas::setup_ui.after(init_editor_settings))
        .add_systems(Update, gui::toolbar::button_system)
        .add_systems(Update, gui::file_panel::file_button_system)
        .add_systems(Update, gui::file_panel::manage_file_button_panel)
        .run();
}
