use bevy::prelude::*;
use crate::editor_settings::EditorSettings;

#[derive(Component)]
pub struct Hierarchy;

pub fn spawn_hierarchy(commands: &mut Commands, settings: &Res<EditorSettings>) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Percent(80.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(60.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.0)),
                    margin: UiRect::all(Val::Percent(0.5)),
                    border: UiRect::bottom(Val::Px(1.0)),
                    ..default()
                },
                border_radius: BorderRadius::new (
                    Val::Px(4.8),
                    Val::Px(4.8),
                    Val::Px(4.8),
                    Val::Px(4.8),
                ),
                background_color: settings.sub_panel_background.into(),
                ..default()
            },
            Hierarchy,
        ))
        .id()
}