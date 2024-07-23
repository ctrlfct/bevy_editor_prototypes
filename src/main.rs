use bevy::prelude::*;

mod toolbar;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, toolbar::setup_toolbar, toolbar::toggle_file_menu))
        .run()
}
