use bevy::prelude::*;
use bevy_editor_prototypes::editor_settings::EditorSettings; 

use bevy_editor_prototypes::gui;
use bevy_editor_prototypes::editor;

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
    .add_systems(Update, editor::new_project::new_project_system)
    .add_systems(Update, gui::project_manager::open_project_selector_system)
    .add_systems(Update, gui::project_manager::setup_ui.after(gui::project_manager::open_project_selector_system))
    .add_systems(Update, gui::new_project::open_project_creator_system)
    .add_systems(Update, gui::new_project::setup_ui.after(gui::new_project::open_project_creator_system))
    .add_systems(Update, editor::close_editor::close_editor_system)
    .run();
}
