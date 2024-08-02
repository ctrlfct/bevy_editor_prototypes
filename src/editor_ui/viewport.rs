use bevy::prelude::*;
use crate::editor_settings::EditorSettings;

#[derive(Component)]
pub struct Viewport;

pub fn spawn_viewport(commands: &mut Commands, settings: &Res<EditorSettings>) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                
                border_radius: BorderRadius::new (
                    Val::Px(4.8),
                    Val::Px(4.8),
                    Val::Px(4.8),
                    Val::Px(4.8),
                ),
                background_color: Color::srgb(0.1, 0.1, 0.1).into(),
                ..default()
            },
            Viewport,
        ))
        .id()
}