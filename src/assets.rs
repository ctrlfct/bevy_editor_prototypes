use bevy::prelude::*;
use crate::editor_settings::EditorSettings;

#[derive(Component)]
pub struct AssetPanel;

pub fn spawn_asset_panel(commands: &mut Commands, settings: &Res<EditorSettings>) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
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
            AssetPanel,
        ))
        .id()
}