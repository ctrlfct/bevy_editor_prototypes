use bevy::{prelude::*, render::camera::RenderTarget, window::WindowRef};
use crate::editor_settings::EditorSettings;
use crate::gui::ui_components::{
    FileButtonsAction, 
    ProjectNameInputFieldAction, 
    ProjectPathInputFieldAction,
    NewProjectButtonsAction,};
use crate::editor::create_project;
use crate::gui::project_manager;
use crate::gui::button::{create_button, ButtonOrientation};
use crate::gui::input_field::create_input_field;

#[derive(Component)]
pub struct ProjectCreator;

#[derive(Component)]
pub struct ProjectCreatorCamera;

pub fn handle_project_creator(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &FileButtonsAction), (Changed<Interaction>, With<Button>)>,
    asset_server: Res<AssetServer>,
    settings: Res<EditorSettings>,
) {
    for (interaction, action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed && matches!(action, FileButtonsAction::New) {
            let project_creator_window = commands
                .spawn(Window {
                    title: "Create new project".to_owned(),
                    ..default()
                })
                .id();
            
            let project_creator_camera = commands
                .spawn((
                    Camera2dBundle {
                        camera: Camera {
                            target: RenderTarget::Window(WindowRef::Entity(project_creator_window)),
                            ..default()
                        },
                        ..default()
                    },
                    ProjectCreatorCamera
                ))
                .id();

            setup_ui(&mut commands, &asset_server, &settings, project_creator_camera);
        }
    }
}

fn setup_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    settings: &Res<EditorSettings>,
    project_creator_camera: Entity,
) {
    let canvas_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::fr(1.0), GridTrack::auto(), GridTrack::fr(1.0)],
                    grid_template_rows: vec![GridTrack::fr(1.0), GridTrack::auto(), GridTrack::auto(), GridTrack::auto(), GridTrack::fr(1.0)],
                    ..default()
                },
                background_color: settings.sub_panel_background.into(),
                ..default()
            },
            ProjectCreator,
            TargetCamera(project_creator_camera),
        ))
        .id();

    let project_name_entity = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            create_input_field(parent, settings, "Project Name", ProjectNameInputFieldAction);
        })
        .id();

    let project_path_entity = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            create_input_field(parent, settings, "Project Path", ProjectPathInputFieldAction);
        })
        .id();

    let buttons_entity = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            create_button(parent, settings, "Cancel", NewProjectButtonsAction::Cancel, ButtonOrientation::Horizontal);
            create_button(parent, settings, "Create", NewProjectButtonsAction::Create, ButtonOrientation::Horizontal);
        })
        .id();

    commands.entity(canvas_entity).push_children(&[project_name_entity, project_path_entity, buttons_entity]);

    commands.entity(project_name_entity).insert(Style {
        grid_column: GridPlacement::start(2),
        grid_row: GridPlacement::start(2),
        ..default()
    });

    commands.entity(project_path_entity).insert(Style {
        grid_column: GridPlacement::start(2),
        grid_row: GridPlacement::start(3),
        ..default()
    });

    commands.entity(buttons_entity).insert(Style {
        grid_column: GridPlacement::start(2),
        grid_row: GridPlacement::start(4),
        ..default()
    });
}

