use bevy::prelude::*;
use crate::editor_settings::EditorSettings;
use crate::toolbar;
use crate::file_panel;
use crate::viewport;
use crate::hierarchy;
use crate::assets;

#[derive(Component)]
pub struct MainCanvas;

pub fn spawn_main_canvas(commands: &mut Commands, settings: &Res<EditorSettings>) -> Entity {
    commands
        .spawn((
            NodeBundle {
                z_index: ZIndex::Local(1),
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(5.0)),
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
    let file_panel_entity = file_panel::spawn_file_panel(&mut commands, &settings);
    let viewport_entity = viewport::spawn_viewport(&mut commands, &settings);
    let hierarchy_entity = hierarchy::spawn_hierarchy(&mut commands, &settings);
    let asset_panel_entity = assets::spawn_asset_panel(&mut commands, &settings);
    
    commands.entity(canvas_entity).push_children(&[file_panel_entity, viewport_entity, hierarchy_entity, asset_panel_entity]);
    
    commands.entity(canvas_entity).with_children(|parent| {
        toolbar::setup_toolbar(parent, &asset_server, &settings);
    });
}
