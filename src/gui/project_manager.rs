use bevy::{prelude::*, render::camera::RenderTarget, window::WindowRef};
use crate::editor_settings::EditorSettings;
use crate::gui::ui_components::FileButtonsAction;
use crate::editor::create_project;
use crate::gui::project_manager;
use crate::gui::button::{create_button, ButtonOrientation};
use crate::gui::input_field::create_input_field;

#[derive(Component)]
pub struct ProjectSelector;

#[derive(Component)]
pub struct ProjectManagerCamera;

pub fn open_project_selector_system(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &FileButtonsAction), (Changed<Interaction>, With<Button>)>,
    asset_server: Res<AssetServer>,
    settings: Res<EditorSettings>,
) {
    for (interaction, action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed && matches!(action, FileButtonsAction::Open) {
            let project_selector_window = commands
                .spawn(Window {
                    title: "Project manager".to_owned(),
                    ..default()
                })
                .id();
            
            let project_manager_camera = commands
                .spawn((
                    Camera2dBundle {
                        camera: Camera {
                            target: RenderTarget::Window(WindowRef::Entity(project_selector_window)),
                            ..default()
                        },
                        ..default()
                    },
                    ProjectManagerCamera
                ))
                .id();
            
            setup_ui(&mut commands, &asset_server, &settings, project_manager_camera);
        }
    }
}

pub fn setup_ui(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>, 
    settings: &Res<EditorSettings>,
    project_manager_camera: Entity,
) {
    let canvas_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::fr(2.0), GridTrack::fr(1.0)],
                    grid_template_rows: vec![GridTrack::px(50.0), GridTrack::fr(1.0)],
                    column_gap: Val::Px(5.0),
                    row_gap: Val::Px(5.0),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                background_color: settings.sub_panel_background.into(),
                ..default()
            },
            ProjectSelector,
            TargetCamera(project_manager_camera),
        ))
        .id();

    let top_panel = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            border_radius: BorderRadius::new (
                Val::Px(4.8),
                Val::Px(4.8),
                Val::Px(4.8),
                Val::Px(4.8),
            ),
            background_color: settings.panel_background.into(),
            ..default()
        })
        .id();

    let left_panel = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            border_radius: BorderRadius::new (
                Val::Px(4.8),
                Val::Px(4.8),
                Val::Px(4.8),
                Val::Px(4.8),
            ),
            background_color: settings.panel_background.into(),
            ..default()
        })
        .id();

    let right_panel = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            border_radius: BorderRadius::new (
                Val::Px(4.8),
                Val::Px(4.8),
                Val::Px(4.8),
                Val::Px(4.8),
            ),
            background_color: settings.panel_background.into(),
            ..default()
        })
        .id();

    commands.entity(canvas_entity).push_children(&[top_panel, left_panel, right_panel]);

    commands.entity(top_panel).insert(Style {
        grid_column: GridPlacement::span(2),
        grid_row: GridPlacement::start(1),
        ..default()
    });

    commands.entity(left_panel).insert(Style {
        grid_column: GridPlacement::start(1),
        grid_row: GridPlacement::start(2),
        ..default()
    });

    commands.entity(right_panel).insert(Style {
        grid_column: GridPlacement::start(2),
        grid_row: GridPlacement::start(2),
        ..default()
    });
}

