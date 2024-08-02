use bevy::prelude::*;
use crate::editor_settings::EditorSettings;

#[derive(Component)]
pub struct Inspector;

pub fn spawn_inspector(commands: &mut Commands, settings: &Res<EditorSettings>) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.0)),
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
            },
            Inspector,
        ))
        .id()
}