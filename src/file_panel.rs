use bevy::prelude::*;

use bevy::color::palettes::basic::*;

use crate::button::{create_button, ButtonOrientation};
use crate::editor_settings::EditorSettings;
use crate::ui_components::MenuButtonsAction;
use crate::ui_components::FileButtonsAction;
use crate::ui_components::PlayerButtonsAction;

#[derive(Component)]
pub struct FileButtonPanelVisible(bool);

pub fn file_button_system(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &MenuButtonsAction), (Changed<Interaction>, With<Button>)>,
    mut panel_query: Query<(Entity, &mut FileButtonPanelVisible)>,
) {
    for (interaction, action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed && matches!(action, MenuButtonsAction::File) {
            if let Ok((panel_entity, mut panel_visible)) = panel_query.get_single_mut() {
                panel_visible.0 = !panel_visible.0;
            }
        }
    }
}

pub fn spawn_file_panel(commands: &mut Commands, settings: &Res<EditorSettings>) -> Entity {
    commands
        .spawn((
            NodeBundle {
                z_index: ZIndex::Local(1),
                style: Style {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    top: Val::Px(60.0),
                    left: Val::Px(10.0),
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
            FileButtonPanelVisible(false),
        ))
        .id()
}

fn spawn_file_buttons(parent: &mut ChildBuilder, editor_settings: &Res<EditorSettings>) {
    create_button(parent, editor_settings, "New", FileButtonsAction::New, ButtonOrientation::Vertical);
    create_button(parent, editor_settings, "Open", FileButtonsAction::Open, ButtonOrientation::Vertical);
    create_button(parent, editor_settings, "Save", FileButtonsAction::Save, ButtonOrientation::Vertical);
    create_button(parent, editor_settings, "Save as", FileButtonsAction::SaveAs, ButtonOrientation::Vertical);
    create_button(parent, editor_settings, "Close", FileButtonsAction::Close, ButtonOrientation::Vertical);
}

pub fn manage_file_button_panel(
    mut commands: Commands,
    panel_query: Query<(Entity, &FileButtonPanelVisible), Changed<FileButtonPanelVisible>>,
    editor_settings: Res<EditorSettings>,
) {
    for (entity, visible) in panel_query.iter() {
        if visible.0 {
            commands.entity(entity).with_children(|parent| {
                spawn_file_buttons(parent, &editor_settings);
            });
        } else {
            commands.entity(entity).despawn_descendants();
        }
    }
}