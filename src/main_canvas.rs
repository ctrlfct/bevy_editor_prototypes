use bevy::prelude::*;
use crate::editor_settings::EditorSettings;
use crate::toolbar;
use crate::file_panel;
use crate::viewport;
use crate::hierarchy;
use crate::assets;
use crate::inspector;

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
    let canvas_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::fr(1.0), GridTrack::fr(3.0), GridTrack::fr(1.0)],
                    grid_template_rows: vec![GridTrack::px(50.0), GridTrack::fr(1.0), GridTrack::px(160.0)],
                    ..default()
                },
                background_color: settings.panel_background.into(),
                ..default()
            },
            MainCanvas,
        ))
        .id();

    let viewport_entity = viewport::spawn_viewport(&mut commands, &settings);
    let hierarchy_entity = hierarchy::spawn_hierarchy(&mut commands, &settings);
    let asset_panel_entity = assets::spawn_asset_panel(&mut commands, &settings);
    let inspector_entity = inspector::spawn_inspector(&mut commands, &settings);

    commands.entity(canvas_entity).push_children(&[viewport_entity, hierarchy_entity, asset_panel_entity, inspector_entity]);

    commands.entity(canvas_entity).with_children(|parent| {
        toolbar::setup_toolbar(parent, &asset_server, &settings);
    });

    commands.entity(hierarchy_entity).insert(Style {
        grid_column: GridPlacement::start(1),
        grid_row: GridPlacement::start(2),
        ..default()
    });

    commands.entity(viewport_entity).insert(Style {
        grid_column: GridPlacement::start(2),
        grid_row: GridPlacement::start(2),
        ..default()
    });

    commands.entity(inspector_entity).insert(Style {
        grid_column: GridPlacement::start(3),
        grid_row: GridPlacement::start(2),
        ..default()
    });

    commands.entity(asset_panel_entity).insert(Style {
        grid_column: GridPlacement::span(3),
        grid_row: GridPlacement::start(3),
        ..default()
    });
}
